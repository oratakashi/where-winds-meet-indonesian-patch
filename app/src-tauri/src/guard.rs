//! The decision logic: given the hash of an installed locale file, what may be done to it.
//!
//! | installed file is…                         | action                                   |
//! |--------------------------------------------|------------------------------------------|
//! | the cached patched build                   | nothing (or install a newer translation) |
//! | the original that build was made from      | reinstall from cache ("restored")        |
//! | the original of the latest release         | download, verify, install                |
//! | anything else                              | leave it alone, wait for a release       |
//!
//! `original_sha256` in the release manifest is the only proof that a build belongs to a
//! given game file, so nothing is ever installed over a file whose hash is unknown —
//! except on an explicit, user-confirmed force install.
use crate::model::{DirReport, Event, FileEntry, FileReport, Manifest, Profile, Status};
use crate::remote::Remote;
use crate::store::{Config, GuardDir, State, Store, copy_atomic, entry_key, sha256_file};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

/// How often to look for a newer translation while everything is patched.
const UPDATE_TTL: Duration = Duration::from_secs(6 * 3600);
/// How often to look for a matching release while a file is unknown.
const UNKNOWN_TTL: Duration = Duration::from_secs(10 * 60);
/// A file modified more recently than this may still be mid-write by the patcher.
const SETTLE: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Default, Debug)]
pub struct Opts {
    pub paused: bool,
    pub auto_update: bool,
    /// Ignore the cached release manifest (the user pressed "check now").
    pub force_remote: bool,
}

type Outcome = (Status, String, Option<Event>);

pub struct Guard<R: Remote> {
    store: Store,
    pub(crate) remote: R,
    state: State,
    dirty: bool,
    hashes: HashMap<PathBuf, (u64, Option<SystemTime>, String)>,
    manifests: HashMap<Profile, (Instant, Result<Manifest, String>)>,
    pub(crate) settle: Duration,
}

impl<R: Remote> Guard<R> {
    pub fn new(store: Store, remote: R) -> Guard<R> {
        let state = store.load_state();
        Guard {
            store,
            remote,
            state,
            dirty: false,
            hashes: HashMap::new(),
            manifests: HashMap::new(),
            settle: SETTLE,
        }
    }

    /// Check every guarded file once. `force_dir`: install the latest release into that
    /// folder even over files whose hash is unknown.
    pub fn check_all(
        &mut self,
        cfg: &Config,
        mut opts: Opts,
        force_dir: Option<&Path>,
    ) -> (Vec<DirReport>, Vec<Event>) {
        opts.paused = cfg.paused;
        opts.auto_update = cfg.auto_update;
        if opts.force_remote {
            self.manifests.clear();
        }
        let mut reports = Vec::new();
        let mut events = Vec::new();
        for d in &cfg.dirs {
            let force = force_dir == Some(d.path.as_path());
            let mut files = Vec::new();
            for name in d.profile.files() {
                let (status, detail, ev) = self.check_file(d.profile, &d.path, name, opts, force);
                events.extend(ev);
                files.push(FileReport { name: name.to_string(), status, detail });
            }
            reports.push(DirReport {
                profile: d.profile,
                path: d.path.to_string_lossy().into_owned(),
                files,
            });
        }
        self.flush();
        (reports, events)
    }

    /// Put back whatever each guarded file held before the guard first replaced it.
    /// Returns what was restored and what could not be.
    pub fn uninstall(&mut self, dirs: &[GuardDir]) -> (Vec<Event>, Vec<String>) {
        let mut events = Vec::new();
        let mut errors = Vec::new();
        for d in dirs {
            for name in d.profile.files() {
                let path = d.path.join(name);
                let pkey = path_key(&path);
                let Some(backup) = self.state.backups.get(&pkey).cloned() else { continue };
                let src = self.store.original_path(&backup);
                if !src.is_file() || !path.is_file() {
                    continue;
                }
                match sha256_file(&path) {
                    Ok(cur) if cur == backup => {}
                    Ok(_) => match copy_atomic(&src, &path) {
                        Ok(()) => {
                            self.hashes.remove(&path);
                            self.state.installed.remove(&pkey);
                            self.dirty = true;
                            events.push(Event::Uninstalled {
                                profile: d.profile,
                                name: name.to_string(),
                            });
                        }
                        Err(e) => errors.push(format!("{}: {e}", path.display())),
                    },
                    Err(e) => errors.push(format!("{}: {e}", path.display())),
                }
            }
        }
        self.flush();
        (events, errors)
    }

    fn flush(&mut self) {
        if self.dirty {
            self.dirty = false;
            let _ = self.store.save_state(&self.state);
            self.store.prune(&self.state);
        }
    }

    fn check_file(
        &mut self,
        profile: Profile,
        dir: &Path,
        name: &str,
        opts: Opts,
        force_install: bool,
    ) -> Outcome {
        let path = dir.join(name);
        let meta = match fs::metadata(&path) {
            Ok(m) if m.is_file() => m,
            _ => return (Status::Missing, "file tidak ada di folder ini".into(), None),
        };
        if let Ok(age) = meta.modified().and_then(|t| t.elapsed().map_err(io::Error::other)) {
            if age < self.settle {
                return (Status::Busy, "sedang ditulis — dicek lagi sebentar".into(), None);
            }
        }
        let sha = match self.hash(&path, &meta) {
            Ok(s) => s,
            Err(e) => return (Status::Busy, format!("belum bisa dibaca ({e})"), None),
        };
        let pkey = path_key(&path);
        let local = self.state.entries.get(&entry_key(profile, name)).cloned();

        if !force_install {
            if let Some(l) = &local {
                if sha == l.patched_sha256 {
                    return self.maybe_update(profile, &path, &sha, l, opts);
                }
                if sha == l.original_sha256 {
                    if opts.paused {
                        return (Status::Original, "file asli game (penjaga dijeda)".into(), None);
                    }
                    if self.store.patched_path(&l.patched_sha256).is_file() {
                        return match self.install(&path, &sha, l) {
                            Ok(()) => (
                                Status::Patched,
                                "patch dipasang kembali".into(),
                                Some(Event::Restored { profile, name: name.to_string() }),
                            ),
                            Err(f) => f,
                        };
                    }
                }
            }
            if opts.paused {
                let ours = self.state.installed.get(&pkey) == Some(&sha);
                return if ours {
                    (Status::Patched, "build lama".into(), None)
                } else {
                    (Status::Unknown, "belum diperiksa (penjaga dijeda)".into(), None)
                };
            }
        }

        let r = match self.remote_entry(profile, name, UNKNOWN_TTL) {
            Ok(Some(r)) => r,
            Ok(None) => {
                return (Status::Unknown, "rilis terbaru tidak memuat file ini".into(), None);
            }
            Err(e) => return (Status::Unknown, format!("rilis tidak bisa diperiksa: {e}"), None),
        };
        if sha == r.patched_sha256 {
            // Installed by hand (or by an earlier run): adopt it and keep a cached copy.
            let cache = self.store.patched_path(&r.patched_sha256);
            if !cache.is_file() {
                let _ = copy_atomic(&path, &cache);
            }
            self.state.entries.insert(entry_key(profile, name), r.clone());
            self.state.installed.insert(pkey, r.patched_sha256);
            self.dirty = true;
            return (Status::Patched, String::new(), None);
        }
        if sha == r.original_sha256 || force_install {
            return match self.fetch_and_install(profile, &path, &sha, &r) {
                Ok(()) => (
                    Status::Patched,
                    "patch dipasang".into(),
                    Some(Event::Installed { profile, name: name.to_string() }),
                ),
                Err(f) => f,
            };
        }
        if self.state.installed.get(&pkey) == Some(&sha) {
            return (
                Status::Patched,
                "build lama — file game di folder ini beda versi dari rilis terbaru".into(),
                None,
            );
        }
        (
            Status::Unknown,
            "versi file tidak cocok dengan rilis terbaru — dibiarkan, menunggu rilis baru".into(),
            None,
        )
    }

    /// The file is the cached patched build: install a newer translation of the same
    /// original if there is one.
    fn maybe_update(
        &mut self,
        profile: Profile,
        path: &Path,
        sha: &str,
        local: &FileEntry,
        opts: Opts,
    ) -> Outcome {
        self.state.installed.entry(path_key(path)).or_insert_with(|| {
            self.dirty = true;
            sha.to_string()
        });
        if opts.paused || !opts.auto_update {
            return (Status::Patched, String::new(), None);
        }
        let Ok(Some(r)) = self.remote_entry(profile, &local.name, UPDATE_TTL) else {
            return (Status::Patched, String::new(), None);
        };
        if r.original_sha256 != local.original_sha256 || r.patched_sha256 == local.patched_sha256 {
            return (Status::Patched, String::new(), None);
        }
        match self.fetch_and_install(profile, path, sha, &r) {
            Ok(()) => (
                Status::Patched,
                "terjemahan diperbarui".into(),
                Some(Event::Installed { profile, name: local.name.clone() }),
            ),
            Err((_, why, _)) => (Status::Patched, format!("pembaruan terjemahan gagal: {why}"), None),
        }
    }

    fn fetch_and_install(
        &mut self,
        profile: Profile,
        path: &Path,
        cur_sha: &str,
        r: &FileEntry,
    ) -> Result<(), Outcome> {
        let cache = self.store.patched_path(&r.patched_sha256);
        if !cache.is_file() {
            let part = self.store.download_path(&r.patched_sha256);
            let got = self
                .remote
                .download(profile, r, &part)
                .and_then(|()| sha256_file(&part).map_err(|e| e.to_string()))
                .and_then(|sha| {
                    if sha == r.patched_sha256 {
                        fs::rename(&part, &cache).map_err(|e| e.to_string())
                    } else {
                        // The rolling release can be replaced mid-download: just retry later.
                        Err("unduhan tidak cocok dengan manifest".to_string())
                    }
                });
            if let Err(e) = got {
                let _ = fs::remove_file(&part);
                self.manifests.remove(&profile);
                return Err((Status::Error, format!("gagal mengunduh: {e}"), None));
            }
        }
        self.install(path, cur_sha, r)?;
        self.state.entries.insert(entry_key(profile, &r.name), r.clone());
        Ok(())
    }

    /// Replace `path` (currently `cur_sha`) with the cached patched build of `entry`.
    fn install(&mut self, path: &Path, cur_sha: &str, entry: &FileEntry) -> Result<(), Outcome> {
        let pkey = path_key(path);
        let ours = self.state.installed.get(&pkey).is_some_and(|s| s == cur_sha);
        if !ours && cur_sha != entry.patched_sha256 {
            let backup = self.store.original_path(cur_sha);
            if !backup.is_file() {
                copy_atomic(path, &backup).map_err(write_failure)?;
            }
            self.state.backups.insert(pkey.clone(), cur_sha.to_string());
            self.dirty = true;
        }
        copy_atomic(&self.store.patched_path(&entry.patched_sha256), path).map_err(write_failure)?;
        self.hashes.remove(path);
        self.state.installed.insert(pkey, entry.patched_sha256.clone());
        self.dirty = true;
        Ok(())
    }

    fn remote_entry(
        &mut self,
        profile: Profile,
        name: &str,
        max_age: Duration,
    ) -> Result<Option<FileEntry>, String> {
        let fresh = self.manifests.get(&profile).is_some_and(|(at, _)| at.elapsed() < max_age);
        if !fresh {
            let m = self.remote.manifest(profile);
            self.manifests.insert(profile, (Instant::now(), m));
        }
        match &self.manifests[&profile].1 {
            Ok(m) => Ok(m.entry(name).cloned()),
            Err(e) => Err(e.clone()),
        }
    }

    /// sha256 of `path`, recomputed only when its size or mtime changed (the files are
    /// 50–85 MB and this runs on every poll).
    fn hash(&mut self, path: &Path, meta: &fs::Metadata) -> io::Result<String> {
        let stamp = (meta.len(), meta.modified().ok());
        if let Some((len, mtime, sha)) = self.hashes.get(path) {
            if (*len, *mtime) == stamp {
                return Ok(sha.clone());
            }
        }
        let sha = sha256_file(path)?;
        self.hashes.insert(path.to_path_buf(), (stamp.0, stamp.1, sha.clone()));
        Ok(sha)
    }
}

fn path_key(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// A failed write is almost always the game or the patcher holding the file: retry.
fn write_failure(e: io::Error) -> Outcome {
    (Status::Busy, format!("belum bisa ditulis ({e}) — dicoba lagi"), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::MANIFEST_SCHEMA;
    use sha2::{Digest, Sha256};
    use std::cell::{Cell, RefCell};

    const NAME: &str = "translate_words_map_en";

    fn sha(b: &[u8]) -> String {
        Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
    }

    #[derive(Default)]
    struct Fake {
        /// (original, patched) the "release" was built from; None = no manifest published.
        build: RefCell<Option<(Vec<u8>, Vec<u8>)>>,
        /// What the download actually returns (to simulate a corrupt/replaced asset).
        served: RefCell<Option<Vec<u8>>>,
        downloads: Cell<u32>,
        fetches: Cell<u32>,
    }

    impl Fake {
        fn publish(&self, original: &[u8], patched: &[u8]) {
            *self.build.borrow_mut() = Some((original.to_vec(), patched.to_vec()));
            *self.served.borrow_mut() = Some(patched.to_vec());
        }
    }

    impl Remote for Fake {
        fn manifest(&self, _: Profile) -> Result<Manifest, String> {
            self.fetches.set(self.fetches.get() + 1);
            let b = self.build.borrow();
            let (o, p) = b.as_ref().ok_or("rilis belum punya manifest.json")?;
            Ok(Manifest {
                schema: MANIFEST_SCHEMA,
                profile: "global".into(),
                commit: String::new(),
                built_at: String::new(),
                files: vec![FileEntry {
                    name: NAME.into(),
                    original_sha256: sha(o),
                    original_size: o.len() as u64,
                    patched_sha256: sha(p),
                    patched_size: p.len() as u64,
                }],
            })
        }

        fn download(&self, _: Profile, _: &FileEntry, dest: &Path) -> Result<(), String> {
            self.downloads.set(self.downloads.get() + 1);
            fs::write(dest, self.served.borrow().as_ref().unwrap()).map_err(|e| e.to_string())
        }
    }

    struct Rig {
        _tmp: tempfile::TempDir,
        guard: Guard<Fake>,
        cfg: Config,
        file: PathBuf,
    }

    impl Rig {
        fn new(installed: &[u8]) -> Rig {
            let tmp = tempfile::tempdir().unwrap();
            let game = tmp.path().join("locale");
            fs::create_dir_all(&game).unwrap();
            let file = game.join(NAME);
            fs::write(&file, installed).unwrap();
            let store = Store::new(tmp.path().join("data")).unwrap();
            let mut guard = Guard::new(store, Fake::default());
            guard.settle = Duration::ZERO;
            let cfg = Config {
                dirs: vec![GuardDir { profile: Profile::Global, path: game }],
                ..Config::default()
            };
            Rig { _tmp: tmp, guard, cfg, file }
        }

        /// Status + detail of the main file, and the events of one check.
        fn check(&mut self) -> (Status, Vec<Event>) {
            self.check_with(Opts { force_remote: true, ..Opts::default() }, false)
        }

        fn check_with(&mut self, opts: Opts, force: bool) -> (Status, Vec<Event>) {
            let dir = self.cfg.dirs[0].path.clone();
            let (reps, evs) = self.guard.check_all(&self.cfg, opts, force.then_some(dir.as_path()));
            (reps[0].files[0].status, evs)
        }

        fn content(&self) -> Vec<u8> {
            fs::read(&self.file).unwrap()
        }

        /// The patcher overwrites the file. Bump mtime so the hash cache sees it even on
        /// filesystems with coarse timestamps.
        fn overwrite(&self, data: &[u8]) {
            fs::write(&self.file, data).unwrap();
            let f = fs::File::options().write(true).open(&self.file).unwrap();
            f.set_modified(SystemTime::now() - Duration::from_secs(data.len() as u64 + 60)).unwrap();
        }
    }

    fn installed() -> Event {
        Event::Installed { profile: Profile::Global, name: NAME.into() }
    }

    #[test]
    fn original_matching_release_is_patched() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v1");
        // Nothing more to do on the next pass.
        assert_eq!(r.check(), (Status::Patched, vec![]));
        assert_eq!(r.guard.remote.downloads.get(), 1);
    }

    #[test]
    fn reverted_file_is_restored_from_cache_without_network() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        r.overwrite(b"orig-v1");
        *r.guard.remote.build.borrow_mut() = None; // release unreachable
        let fetches = r.guard.remote.fetches.get();
        let (status, evs) = r.check_with(Opts::default(), false);
        assert_eq!(status, Status::Patched);
        assert_eq!(evs, vec![Event::Restored { profile: Profile::Global, name: NAME.into() }]);
        assert_eq!(r.content(), b"patched-v1");
        assert_eq!(r.guard.remote.downloads.get(), 1);
        assert_eq!(r.guard.remote.fetches.get(), fetches);
    }

    #[test]
    fn unknown_file_is_never_touched() {
        let mut r = Rig::new(b"orig-v2-new-game-version");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        assert_eq!(r.check(), (Status::Unknown, vec![]));
        assert_eq!(r.content(), b"orig-v2-new-game-version");
        assert_eq!(r.guard.remote.downloads.get(), 0);
    }

    #[test]
    fn missing_manifest_leaves_file_alone() {
        let mut r = Rig::new(b"orig-v1");
        assert_eq!(r.check(), (Status::Unknown, vec![]));
        assert_eq!(r.content(), b"orig-v1");
    }

    #[test]
    fn new_game_version_waits_for_matching_release() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        r.overwrite(b"orig-v2");
        assert_eq!(r.check(), (Status::Unknown, vec![]));
        assert_eq!(r.content(), b"orig-v2", "a stale patch must not be installed");
        r.guard.remote.publish(b"orig-v2", b"patched-v2");
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v2");
    }

    #[test]
    fn hand_installed_patch_is_adopted_and_then_guarded() {
        let mut r = Rig::new(b"patched-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        assert_eq!(r.check(), (Status::Patched, vec![]));
        assert_eq!(r.guard.remote.downloads.get(), 0);
        r.overwrite(b"orig-v1");
        let (status, evs) = r.check();
        assert_eq!((status, evs.len()), (Status::Patched, 1));
        assert_eq!(r.content(), b"patched-v1");
        assert_eq!(r.guard.remote.downloads.get(), 0, "restored from the adopted copy");
    }

    #[test]
    fn corrupt_download_is_rejected() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        *r.guard.remote.served.borrow_mut() = Some(b"something else".to_vec());
        assert_eq!(r.check(), (Status::Error, vec![]));
        assert_eq!(r.content(), b"orig-v1");
        // Fixed on the server: the next pass succeeds.
        *r.guard.remote.served.borrow_mut() = Some(b"patched-v1".to_vec());
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
    }

    #[test]
    fn newer_translation_is_installed_only_with_auto_update() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        r.guard.remote.publish(b"orig-v1", b"patched-v1-more-lines");
        r.cfg.auto_update = false;
        assert_eq!(r.check(), (Status::Patched, vec![]));
        assert_eq!(r.content(), b"patched-v1");
        r.cfg.auto_update = true;
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v1-more-lines");
    }

    #[test]
    fn paused_guard_only_reports() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        r.overwrite(b"orig-v1");
        r.cfg.paused = true;
        assert_eq!(r.check(), (Status::Original, vec![]));
        assert_eq!(r.content(), b"orig-v1");
    }

    #[test]
    fn uninstall_puts_the_original_back() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        let (evs, errs) = r.guard.uninstall(&r.cfg.dirs);
        assert_eq!((evs.len(), errs.len()), (1, 0));
        assert_eq!(r.content(), b"orig-v1");
        // A second uninstall has nothing to do.
        assert_eq!(r.guard.uninstall(&r.cfg.dirs).0.len(), 0);
    }

    #[test]
    fn uninstall_after_game_update_restores_the_newest_original() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        r.overwrite(b"orig-v2");
        r.guard.remote.publish(b"orig-v2", b"patched-v2");
        r.check();
        r.guard.uninstall(&r.cfg.dirs);
        assert_eq!(r.content(), b"orig-v2");
    }

    #[test]
    fn force_install_overrides_an_unknown_file_and_keeps_a_backup() {
        let mut r = Rig::new(b"old patch installed by hand");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        assert_eq!(r.check().0, Status::Unknown);
        let (status, evs) = r.check_with(Opts { force_remote: true, ..Opts::default() }, true);
        assert_eq!((status, evs), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v1");
        r.guard.uninstall(&r.cfg.dirs);
        assert_eq!(r.content(), b"old patch installed by hand");
    }

    #[test]
    fn state_survives_a_restart() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        r.overwrite(b"orig-v1");
        let mut again = Guard::new(r.guard.store.clone(), Fake::default());
        again.settle = Duration::ZERO;
        let (reps, evs) = again.check_all(&r.cfg, Opts::default(), None);
        assert_eq!((reps[0].files[0].status, evs.len()), (Status::Patched, 1));
        assert_eq!(r.content(), b"patched-v1");
    }

    #[test]
    fn recently_modified_file_is_left_to_settle() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.guard.settle = Duration::from_secs(3600);
        assert_eq!(r.check(), (Status::Busy, vec![]));
        assert_eq!(r.content(), b"orig-v1");
    }

    #[cfg(windows)]
    #[test]
    fn file_held_by_the_game_is_retried_not_broken() {
        use std::os::windows::fs::OpenOptionsExt;
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        // FILE_SHARE_READ only: readable, but it cannot be replaced.
        let held = fs::File::options().read(true).share_mode(1).open(&r.file).unwrap();
        assert_eq!(r.check(), (Status::Busy, vec![]));
        assert_eq!(r.content(), b"orig-v1");
        drop(held);
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v1");
    }
}
