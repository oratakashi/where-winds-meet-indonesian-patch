//! The background threads: the guard thread watches the guarded folders and runs the guard
//! whenever a file changes, on a slow poll (in case a change notification is lost), and on
//! request; the update thread looks for a newer Patch Guard and downloads it (see `update.rs`).
use crate::guard::{Guard, Opts};
use crate::locations;
use crate::i18n::{self, Lang};
use crate::model::{DirReport, Event, LogLine, LogMsg, Status};
use crate::remote::HttpRemote;
use crate::store::{Config, Store};
use crate::update::{self, Pending, Updates};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Fallback poll. Costs one `metadata` call per file: hashes are cached by size + mtime.
const POLL: Duration = Duration::from_secs(30);
/// Re-check sooner while a file is still being written or is locked.
const RETRY: Duration = Duration::from_secs(5);
/// After a change notification, wait for the folder to go quiet…
const DEBOUNCE: Duration = Duration::from_secs(2);
/// …but not forever, if the patcher keeps writing other files there.
const DEBOUNCE_MAX: Duration = Duration::from_secs(30);
/// History lines kept (and saved to `history.json`).
const LOG_MAX: usize = 500;
/// First app-update check after start (let the guard's first pass go first)…
const UPDATE_FIRST: Duration = Duration::from_secs(20);
/// …then this often.
const UPDATE_EVERY: Duration = Duration::from_secs(6 * 60 * 60);

type OnChange = Arc<dyn Fn(&[Event]) + Send + Sync>;

pub enum Msg {
    /// Something changed in a watched folder.
    Fs,
    Check { force_remote: bool },
    /// Install the latest release into this folder even over unknown files.
    ForceInstall(PathBuf),
    /// Put the original files back (the caller pauses the guard first).
    Uninstall,
    /// The config changed: re-watch folders and re-check.
    Reload,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Snapshot {
    pub dirs: Vec<DirReport>,
    pub paused: bool,
    pub auto_update: bool,
    pub checking: bool,
    pub last_check: Option<u64>,
    pub log: Vec<LogLine>,
    /// Self-update switch (`Config::auto_update_app`).
    pub auto_update_app: bool,
    /// Install it on quit / next start without asking (`Config::auto_install_app`).
    pub auto_install_app: bool,
    /// A newer Patch Guard is downloaded, verified and waiting: its version.
    pub app_update: Option<String>,
    pub app_update_checking: bool,
    /// Outcome of the last app-update check, for the Settings page.
    pub app_update_note: Option<AppNote>,
    /// The language the dirs' details were last phrased in.
    pub lang: Lang,
}

/// The outcome of an app-update check, phrased when shown (so it follows a language switch).
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub enum AppNote {
    Downloaded(String),
    UpToDate,
    /// The error text (from the network or the release check).
    CheckFailed(String),
    /// This version's installer failed before; not fetched again automatically.
    SkippedFailed(String),
    InstallFailed { version: String, code: Option<i32> },
}

impl Snapshot {
    /// Copy the Settings switches from the saved config.
    fn show_switches(&mut self, cfg: &Config) {
        self.paused = cfg.paused;
        self.auto_update = cfg.auto_update;
        self.auto_update_app = cfg.auto_update_app;
        self.auto_install_app = cfg.auto_install_app;
    }
}

impl AppNote {
    pub fn text(&self) -> String {
        match self {
            AppNote::Downloaded(v) => {
                tf!("engine.version_is_downloaded_and", v = v)
            }
            AppNote::UpToDate => {
                let v = update::CURRENT;
                tf!("engine.you_have_the_latest", v = v)
            }
            AppNote::CheckFailed(e) => tf!("engine.check_failed", e = e),
            AppNote::SkippedFailed(v) => tf!("engine.updating_to_v_failed", v = v),
            AppNote::InstallFailed { version, code } => {
                LogMsg::AppUpdateFailed { version: version.clone(), code: *code }.text()
            }
        }
    }
}

pub struct Engine {
    tx: Sender<Msg>,
    update_tx: Sender<()>,
    updates: Arc<Updates>,
    store: Store,
    config: Arc<Mutex<Config>>,
    shared: Arc<Mutex<Snapshot>>,
}

impl Engine {
    /// `on_change` runs on the engine thread after every pass, with what the guard did.
    /// `first_pass_after`: how long to wait before the first pass (autostart at login waits a
    /// little, so the guard does not add disk and network load while Windows is starting).
    pub fn start(
        store: Store,
        first_pass_after: Duration,
        on_change: impl Fn(&[Event]) + Send + Sync + 'static,
    ) -> Engine {
        let on_change: OnChange = Arc::new(on_change);
        let mut cfg = store.load_config();
        if locations::migrate(&mut cfg.dirs) {
            let _ = store.save_config(&cfg);
        }
        // Before any thread phrases a status detail or a history line.
        i18n::set(cfg.language);
        let shared = Arc::new(Mutex::new(Snapshot {
            lang: cfg.language,
            paused: cfg.paused,
            auto_update: cfg.auto_update,
            checking: !cfg.dirs.is_empty() && first_pass_after.is_zero(),
            log: store.load_history(),
            auto_update_app: cfg.auto_update_app,
            auto_install_app: cfg.auto_install_app,
            ..Snapshot::default()
        }));
        let config = Arc::new(Mutex::new(cfg));
        let (tx, rx) = mpsc::channel();
        let worker = Worker {
            rx,
            tx: tx.clone(),
            store: store.clone(),
            config: config.clone(),
            shared: shared.clone(),
            first_pass_after,
        };
        let guard_change = on_change.clone();
        thread::Builder::new()
            .name("guard".into())
            .spawn(move || worker.run(&*guard_change))
            .expect("spawn guard thread");

        let updates = Arc::new(Updates::new(store.base()));
        let (update_tx, update_rx) = mpsc::channel();
        let updater = Updater {
            rx: update_rx,
            updates: updates.clone(),
            store: store.clone(),
            config: config.clone(),
            shared: shared.clone(),
        };
        thread::Builder::new()
            .name("app-update".into())
            .spawn(move || updater.run(&*on_change))
            .expect("spawn update thread");
        Engine { tx, update_tx, updates, store, config, shared }
    }

    /// Look for a newer Patch Guard now (also when automatic updates are off).
    pub fn check_app_update(&self) {
        let _ = self.update_tx.send(());
    }

    /// The downloaded update waiting to be installed, if any.
    pub fn pending_update(&self) -> Option<Pending> {
        self.updates.pending()
    }

    /// The guard is in the middle of a pass (it may be writing game files).
    pub fn busy(&self) -> bool {
        self.shared.lock().unwrap().checking
    }

    /// Just the waiting update's version (what the tray needs, without copying the history).
    pub fn app_update(&self) -> Option<String> {
        self.shared.lock().unwrap().app_update.clone()
    }

    pub fn snapshot(&self) -> Snapshot {
        self.shared.lock().unwrap().clone()
    }

    pub fn config(&self) -> Config {
        self.config.lock().unwrap().clone()
    }

    pub fn send(&self, msg: Msg) {
        let _ = self.tx.send(msg);
    }

    pub fn clear_log(&self) {
        self.shared.lock().unwrap().log.clear();
        let _ = self.store.save_history(&[]);
    }

    /// Change the config, persist it and make the engine pick it up.
    /// The switches in the snapshot follow at once (the window refreshes from it right after),
    /// not only when the next guard pass ends.
    pub fn update_config(&self, f: impl FnOnce(&mut Config)) -> Result<(), String> {
        let mut cfg = self.config.lock().unwrap();
        f(&mut cfg);
        self.store.save_config(&cfg).map_err(|e| e.to_string())?;
        // Published while the config is still locked. Lock order everywhere: config, then
        // snapshot — so the guard thread, which publishes the same way, cannot slip an older
        // config in between.
        self.shared.lock().unwrap().show_switches(&cfg);
        drop(cfg);
        self.send(Msg::Reload);
        Ok(())
    }
}

#[derive(Default)]
struct Batch {
    fs: bool,
    force_remote: bool,
    force_dir: Option<PathBuf>,
    uninstall: bool,
    reload: bool,
}

impl Batch {
    /// Nothing asked for this pass: the poll timer ran out.
    fn is_routine(&self) -> bool {
        !(self.fs || self.force_remote || self.force_dir.is_some() || self.uninstall || self.reload)
    }

    fn absorb(&mut self, m: Msg) {
        match m {
            Msg::Fs => self.fs = true,
            Msg::Check { force_remote } => self.force_remote |= force_remote,
            Msg::ForceInstall(dir) => {
                self.force_remote = true;
                self.force_dir = Some(dir);
            }
            Msg::Uninstall => self.uninstall = true,
            Msg::Reload => self.reload = true,
        }
    }
}

struct Worker {
    rx: Receiver<Msg>,
    tx: Sender<Msg>,
    store: Store,
    config: Arc<Mutex<Config>>,
    shared: Arc<Mutex<Snapshot>>,
    first_pass_after: Duration,
}

impl Worker {
    fn run(self, on_change: &dyn Fn(&[Event])) {
        background_priority();
        let mut guard = Guard::new(self.store.clone(), HttpRemote::new());
        let tx = self.tx.clone();
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            // Our own temp copies (`.<name>.wwmpg-tmp`) are not news; the rename onto the real
            // name still is.
            if res.is_ok_and(|e| !e.paths.iter().all(|p| is_own_temp(p))) {
                let _ = tx.send(Msg::Fs);
            }
        })
        .ok();
        let mut watched: Vec<PathBuf> = Vec::new();
        let mut first = true;
        let mut wait = self.first_pass_after;
        loop {
            let mut batch = Batch { reload: first, ..Batch::default() };
            first = false;
            match self.rx.recv_timeout(wait) {
                Ok(m) => batch.absorb(m),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => return,
            }
            let started = Instant::now();
            loop {
                let quiet = if batch.fs && started.elapsed() < DEBOUNCE_MAX {
                    DEBOUNCE
                } else {
                    Duration::ZERO
                };
                match self.rx.recv_timeout(quiet) {
                    Ok(m) => batch.absorb(m),
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            }

            let cfg = self.config.lock().unwrap().clone();
            if batch.reload {
                rewatch(watcher.as_mut(), &mut watched, &cfg);
            }
            // A routine poll runs quietly: no "checking" flash and no wake-up of the window
            // before it, only after it (most polls change nothing and cost a few metadata calls).
            if !batch.is_routine() {
                self.shared.lock().unwrap().checking = true;
                on_change(&[]);
            }

            let mut events = Vec::new();
            let mut lines: Vec<(String, Option<LogMsg>)> = Vec::new();
            if batch.uninstall {
                let (ev, errs) = guard.uninstall(&cfg.dirs);
                events.extend(ev);
                lines.extend(
                    errs.into_iter().map(|e| (tf!("engine.could_not_restore", e = e), None)),
                );
            }
            let opts = Opts { force_remote: batch.force_remote, ..Opts::default() };
            let (dirs, ev) = guard.check_all(&cfg, opts, batch.force_dir.as_deref());
            events.extend(ev);
            lines.extend(events.iter().map(|e| {
                let m = LogMsg::from(e);
                (m.text(), Some(m))
            }));

            let busy = dirs.iter().flat_map(|d| &d.files).any(|f| f.status == Status::Busy);
            // A file the game keeps open is retried less and less often (nothing is copied
            // meanwhile); one still being written, soon.
            wait = guard.locked_retry().unwrap_or(if busy { RETRY } else { POLL });

            // The switches as saved now, not as they were when this pass started: a setting
            // changed meanwhile must not be shown with its old value again. The config stays
            // locked until they are published (lock order: config, then snapshot, as in
            // `Engine::update_config`), so no newer save can land in between.
            let latest = self.config.lock().unwrap();
            let mut s = self.shared.lock().unwrap();
            // The details are phrased in the current language: right after a switch every
            // known problem would look new, so that pass logs none.
            let lang = i18n::get();
            lines.extend(problem_lines(&s.dirs, s.lang, &dirs, lang).into_iter().map(|t| (t, None)));
            s.lang = lang;
            let now = unix_now();
            if !lines.is_empty() {
                s.log.extend(lines.into_iter().map(|(text, msg)| LogLine::new(now, text, msg)));
                let extra = s.log.len().saturating_sub(LOG_MAX);
                s.log.drain(..extra);
                let _ = self.store.save_history(&s.log);
            }
            s.dirs = dirs;
            s.show_switches(&latest);
            s.checking = false;
            s.last_check = Some(now);
            drop(s);
            drop(latest);
            on_change(&events);
        }
    }
}

struct Updater {
    rx: Receiver<()>,
    updates: Arc<Updates>,
    store: Store,
    config: Arc<Mutex<Config>>,
    shared: Arc<Mutex<Snapshot>>,
}

impl Updater {
    fn run(self, on_change: &dyn Fn(&[Event])) {
        background_priority();
        let remote = HttpRemote::new();
        // A download from an earlier run may still be waiting.
        self.shared.lock().unwrap().app_update = self.updates.pending().map(|p| p.info.version);
        // A version whose installer failed: not downloaded again automatically, only when
        // the user presses "Periksa update".
        // Kept in `update-failed.json`, so it survives restarts.
        let mut failed_version: Option<String> = self.updates.failed_version();
        if let Some(r) = self.updates.take_result() {
            let msg = if r.ok {
                failed_version = None;
                self.updates.set_failed_version(None);
                LogMsg::AppUpdated { version: r.version.clone() }
            } else {
                failed_version = Some(r.version.clone());
                self.updates.set_failed_version(Some(&r.version));
                LogMsg::AppUpdateFailed { version: r.version.clone(), code: r.code }
            };
            let mut s = self.shared.lock().unwrap();
            if !r.ok {
                s.app_update_note = Some(AppNote::InstallFailed { version: r.version, code: r.code });
            }
            push_log(&mut s, &self.store, msg.text(), Some(msg));
            drop(s);
            on_change(&[]);
        }
        let mut wait = UPDATE_FIRST;
        // The last automatic failure: logged once, not every six hours while offline.
        let mut last_error: Option<String> = None;
        loop {
            let manual = match self.rx.recv_timeout(wait) {
                Ok(()) => true,
                Err(RecvTimeoutError::Timeout) => false,
                Err(RecvTimeoutError::Disconnected) => return,
            };
            wait = UPDATE_EVERY;
            if !manual && !self.config.lock().unwrap().auto_update_app {
                continue;
            }
            if manual && failed_version.take().is_some() {
                self.updates.set_failed_version(None);
            }
            self.shared.lock().unwrap().app_update_checking = true;
            on_change(&[]);

            let mut skipped = None;
            let res = update::check(&remote).and_then(|found| match found {
                Some(rel) if failed_version.as_deref() == Some(rel.info.version.as_str()) => {
                    skipped = Some(rel.info.version);
                    Ok(None)
                }
                Some(rel) => self.updates.fetch(&remote, &rel).map(Some),
                None => Ok(None),
            });
            let mut line: Option<(String, Option<LogMsg>)> = None;
            let (ready, note) = match res {
                Ok(None) if skipped.is_some() => {
                    last_error = None;
                    (None, AppNote::SkippedFailed(skipped.unwrap_or_default()))
                }
                Ok(Some(p)) => {
                    last_error = None;
                    let v = p.info.version;
                    (Some(v.clone()), AppNote::Downloaded(v))
                }
                Ok(None) => {
                    last_error = None;
                    (None, AppNote::UpToDate)
                }
                Err(e) => {
                    if manual || last_error.as_deref() != Some(e.as_str()) {
                        line = Some((
                            tf!("engine.could_not_check_for", e = e),
                            None,
                        ));
                    }
                    last_error = Some(e.clone());
                    (self.updates.pending().map(|p| p.info.version), AppNote::CheckFailed(e))
                }
            };
            let mut s = self.shared.lock().unwrap();
            if let Some(v) = ready.as_ref().filter(|v| s.app_update.as_ref() != Some(*v)) {
                let m = LogMsg::AppUpdateReady { version: v.clone() };
                line = Some((m.text(), Some(m)));
            }
            s.app_update = ready;
            s.app_update_checking = false;
            s.app_update_note = Some(note);
            if let Some((text, msg)) = line {
                push_log(&mut s, &self.store, text, msg);
            }
            drop(s);
            on_change(&[]);
        }
    }
}

/// Append one history line, keep the last `LOG_MAX`, persist.
fn push_log(s: &mut Snapshot, store: &Store, text: String, msg: Option<LogMsg>) {
    s.log.push(LogLine::new(unix_now(), text, msg));
    let extra = s.log.len().saturating_sub(LOG_MAX);
    s.log.drain(..extra);
    let _ = store.save_history(&s.log);
}

fn rewatch(watcher: Option<&mut RecommendedWatcher>, watched: &mut Vec<PathBuf>, cfg: &Config) {
    let Some(w) = watcher else { return };
    for p in watched.drain(..) {
        let _ = w.unwatch(&p);
    }
    for d in &cfg.dirs {
        if w.watch(&d.path, RecursiveMode::NonRecursive).is_ok() {
            watched.push(d.path.clone());
        }
    }
}

/// New problems worth a history line. On the pass right after a language switch every detail
/// is phrased anew, so a changed detail alone does not count then; new files and status
/// changes still do.
fn problem_lines(old: &[DirReport], old_lang: Lang, new: &[DirReport], lang: Lang) -> Vec<String> {
    new_problems(old, new, old_lang == lang)
}

/// Log a file the first time it turns unknown or fails, not on every poll. `same_lang`: also
/// log when only the detail changed (after a language switch every detail is re-phrased, so
/// then only a new file or a status change counts).
fn new_problems(old: &[DirReport], new: &[DirReport], same_lang: bool) -> Vec<String> {
    let mut out = Vec::new();
    for d in new {
        for f in &d.files {
            if !matches!(f.status, Status::Unknown | Status::Error) {
                continue;
            }
            let before = old
                .iter()
                .find(|o| o.path == d.path)
                .and_then(|o| o.files.iter().find(|x| x.name == f.name));
            if before.is_none_or(|b| b.status != f.status || (same_lang && b.detail != f.detail)) {
                out.push(format!("{}: {}", f.name, f.detail));
            }
        }
    }
    out
}

/// `.<name>.wwmpg-tmp`, the temp file `store::copy_atomic` writes next to a game file.
fn is_own_temp(p: &std::path::Path) -> bool {
    p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with('.') && n.ends_with(".wwmpg-tmp"))
}

/// Run this thread at background priority (CPU and disk I/O), so hashing and copying 85 MB
/// files never competes with the game.
fn background_priority() {
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Threading::{
            GetCurrentThread, SetThreadPriority, THREAD_MODE_BACKGROUND_BEGIN,
        };
        let _ = SetThreadPriority(GetCurrentThread(), THREAD_MODE_BACKGROUND_BEGIN);
    }
}

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Client, FileReport, Profile};

    fn dirs(detail: &str) -> Vec<DirReport> {
        vec![DirReport {
            profile: Profile::Global,
            client: Client::Launcher,
            path: r"C:\game\locale".into(),
            files: vec![FileReport {
                name: "translate_words_map_en".into(),
                status: Status::Unknown,
                detail: detail.into(),
            }],
        }]
    }

    #[test]
    fn a_problem_is_logged_once_and_not_again_after_a_language_switch() {
        let (old, new) = (dirs("versi tidak cocok"), dirs("versi tidak cocok"));
        assert_eq!(problem_lines(&[], Lang::Id, &old, Lang::Id).len(), 1, "new problem: logged");
        assert!(problem_lines(&old, Lang::Id, &new, Lang::Id).is_empty(), "same problem: not again");
        let en = dirs("version does not match");
        assert!(problem_lines(&old, Lang::Id, &en, Lang::En).is_empty(), "only re-phrased");
        assert_eq!(problem_lines(&en, Lang::En, &dirs("other"), Lang::En).len(), 1);
    }

    #[test]
    fn a_status_change_during_a_language_switch_is_still_logged() {
        let mut patched = dirs("");
        patched[0].files[0].status = Status::Patched;
        let unknown = dirs("version does not match");
        assert_eq!(problem_lines(&patched, Lang::Id, &unknown, Lang::En).len(), 1);
        // A file not seen before counts as new too.
        assert_eq!(problem_lines(&[], Lang::Id, &unknown, Lang::En).len(), 1);
    }

    #[test]
    fn own_temp_files_and_routine_polls() {
        assert!(is_own_temp(std::path::Path::new(r"C:\g\.translate_words_map_en.wwmpg-tmp")));
        assert!(!is_own_temp(std::path::Path::new(r"C:\g\translate_words_map_en")));
        assert!(Batch::default().is_routine());
        let mut b = Batch::default();
        b.absorb(Msg::Fs);
        assert!(!b.is_routine());
        assert!(!Batch { reload: true, ..Batch::default() }.is_routine());
    }

    #[test]
    fn app_notes_follow_the_language() {
        let n = AppNote::Downloaded("0.6.0".into());
        i18n::set(Lang::Id);
        assert!(n.text().contains("sudah diunduh"));
        i18n::set(Lang::En);
        assert!(n.text().contains("downloaded"));
        assert!(AppNote::InstallFailed { version: "0.6.0".into(), code: None }.text().contains("did not start"));
        i18n::set(Lang::Id);
    }
}
