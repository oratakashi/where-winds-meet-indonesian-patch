//! The background thread: watches the guarded folders and runs the guard whenever a file
//! changes, on a slow poll (in case a change notification is lost), and on request.
use crate::guard::{Guard, Opts};
use crate::model::{DirReport, Event, Status};
use crate::remote::HttpRemote;
use crate::store::{Config, Store};
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
const LOG_MAX: usize = 100;

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

#[derive(Serialize, Clone, Debug)]
pub struct LogLine {
    /// Unix seconds.
    pub at: u64,
    pub text: String,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Snapshot {
    pub dirs: Vec<DirReport>,
    pub paused: bool,
    pub auto_update: bool,
    pub checking: bool,
    pub last_check: Option<u64>,
    pub log: Vec<LogLine>,
}

pub struct Engine {
    tx: Sender<Msg>,
    store: Store,
    config: Arc<Mutex<Config>>,
    shared: Arc<Mutex<Snapshot>>,
}

impl Engine {
    /// `on_change` runs on the engine thread after every pass, with what the guard did.
    pub fn start(store: Store, on_change: impl Fn(&[Event]) + Send + 'static) -> Engine {
        let cfg = store.load_config();
        let shared = Arc::new(Mutex::new(Snapshot {
            paused: cfg.paused,
            auto_update: cfg.auto_update,
            checking: !cfg.dirs.is_empty(),
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
        };
        thread::Builder::new()
            .name("guard".into())
            .spawn(move || worker.run(on_change))
            .expect("spawn guard thread");
        Engine { tx, store, config, shared }
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

    /// Change the config, persist it and make the engine pick it up.
    pub fn update_config(&self, f: impl FnOnce(&mut Config)) -> Result<(), String> {
        let mut cfg = self.config.lock().unwrap();
        f(&mut cfg);
        self.store.save_config(&cfg).map_err(|e| e.to_string())?;
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
}

impl Worker {
    fn run(self, on_change: impl Fn(&[Event])) {
        let mut guard = Guard::new(self.store.clone(), HttpRemote::new());
        let tx = self.tx.clone();
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if res.is_ok() {
                let _ = tx.send(Msg::Fs);
            }
        })
        .ok();
        let mut watched: Vec<PathBuf> = Vec::new();
        let mut first = true;
        let mut wait = Duration::ZERO;
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
            self.shared.lock().unwrap().checking = true;
            on_change(&[]);

            let mut events = Vec::new();
            let mut lines = Vec::new();
            if batch.uninstall {
                let (ev, errs) = guard.uninstall(&cfg.dirs);
                events.extend(ev);
                lines.extend(errs.into_iter().map(|e| format!("Gagal mengembalikan {e}")));
            }
            let opts = Opts { force_remote: batch.force_remote, ..Opts::default() };
            let (dirs, ev) = guard.check_all(&cfg, opts, batch.force_dir.as_deref());
            events.extend(ev);
            lines.extend(events.iter().map(describe));

            let busy = dirs.iter().flat_map(|d| &d.files).any(|f| f.status == Status::Busy);
            wait = if busy { RETRY } else { POLL };

            let mut s = self.shared.lock().unwrap();
            lines.extend(new_problems(&s.dirs, &dirs));
            let now = unix_now();
            s.log.extend(lines.into_iter().map(|text| LogLine { at: now, text }));
            let extra = s.log.len().saturating_sub(LOG_MAX);
            s.log.drain(..extra);
            s.dirs = dirs;
            s.paused = cfg.paused;
            s.auto_update = cfg.auto_update;
            s.checking = false;
            s.last_check = Some(now);
            drop(s);
            on_change(&events);
        }
    }
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

fn describe(e: &Event) -> String {
    match e {
        Event::Restored { name, .. } => format!("{name}: ditimpa patcher, patch dipasang kembali"),
        Event::Installed { name, .. } => format!("{name}: build terbaru dipasang"),
        Event::Uninstalled { name, .. } => format!("{name}: file asli dikembalikan"),
    }
}

/// Log a file the first time it turns unknown or fails, not on every poll.
fn new_problems(old: &[DirReport], new: &[DirReport]) -> Vec<String> {
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
            if before.is_none_or(|b| b.status != f.status || b.detail != f.detail) {
                out.push(format!("{}: {}", f.name, f.detail));
            }
        }
    }
    out
}

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
