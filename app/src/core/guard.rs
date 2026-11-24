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
use crate::i18n::{self, Lang};
use crate::model::{Client, DirReport, Event, FileEntry, FileReport, Manifest, Profile, Status};
use crate::remote::{Remote, extract};
use crate::store::{
    Config, GuardDir, HashCache, HashEntry, State, Store, copy_atomic, entry_key, sha256_file,
};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// How often to look for a newer translation while everything is patched.
const UPDATE_TTL: Duration = Duration::from_secs(6 * 3600);
/// How often to look for a matching release while a file is unknown.
const UNKNOWN_TTL: Duration = Duration::from_secs(10 * 60);
/// A failed manifest fetch (offline, release mid-update) is retried this soon, whatever the
/// TTL of a good one.
const ERROR_TTL: Duration = Duration::from_secs(60);
/// A file modified more recently than this may still be mid-write by the patcher.
const SETTLE: Duration = Duration::from_secs(2);
/// A failed manifest fetch is retried after `error_ttl`, then twice as long each time it fails
/// again, up to this (offline for hours = one attempt every 10 min, not every minute).
const ERROR_TTL_MAX: Duration = Duration::from_secs(10 * 60);
/// A game file the game or patcher keeps open is retried after this, doubling while it stays
/// locked...
const LOCKED_RETRY: Duration = Duration::from_secs(5);
/// ...up to this. Nothing is copied while it is locked.
const LOCKED_RETRY_MAX: Duration = Duration::from_secs(5 * 60);

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
    /// sha256 per path (`hashes.json`), so neither a poll nor a restart re-reads a file whose
    /// size and times did not change.
    hashes: HashCache,
    hashes_dirty: bool,
    /// The last manifest fetch per profile, when it happened, and the language a failure was
    /// phrased in (a failure is not reused after a language switch).
    manifests: HashMap<Profile, (Instant, Result<Manifest, String>, Lang)>,
    /// Failed manifest fetches in a row per profile (the retry backs off).
    manifest_failures: HashMap<Profile, u32>,
    /// A game file was locked in this pass / in this many passes in a row.
    locked_now: bool,
    locked_streak: u32,
    pub(crate) settle: Duration,
    pub(crate) error_ttl: Duration,
}

impl<R: Remote> Guard<R> {
    pub fn new(store: Store, remote: R) -> Guard<R> {
        let state = store.load_state();
        let hashes = store.load_hashes();
        Guard {
            store,
            remote,
            state,
            dirty: false,
            hashes,
            hashes_dirty: false,
            manifests: HashMap::new(),
            manifest_failures: HashMap::new(),
            locked_now: false,
            locked_streak: 0,
            settle: SETTLE,
            error_ttl: ERROR_TTL,
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
            self.manifest_failures.clear();
        }
        self.locked_now = false;
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
                client: Client::of(&d.path),
                path: d.path.to_string_lossy().into_owned(),
                files,
            });
        }
        // The zip only serves the files fetched in this pass (both come from one download).
        self.store.clear_archives();
        self.locked_streak = if self.locked_now { self.locked_streak + 1 } else { 0 };
        self.flush();
        (reports, events)
    }

    /// How soon to look again because a game file was locked: 5 s, doubling each pass it stays
    /// locked, at most 5 min. `None` when nothing was locked in the last pass.
    pub fn locked_retry(&self) -> Option<Duration> {
        let n = self.locked_streak.checked_sub(1)?;
        Some(LOCKED_RETRY.saturating_mul(1 << n.min(16)).min(LOCKED_RETRY_MAX))
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
                            self.forget_hash(&path);
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

    /// Persist the state. A failed write keeps it dirty, so the next pass tries again (the
    /// rollback checkpoint and the backups must not be lost); the cache is only pruned against
    /// a state that is on disk.
    fn flush(&mut self) {
        if self.dirty && self.store.save_state(&self.state).is_ok() {
            self.dirty = false;
            self.store.prune(&self.state);
        }
        if self.hashes_dirty {
            // Forget files that are gone (a removed folder, a pruned cache copy): the map must
            // not grow over months of running.
            self.hashes.retain(|p, _| Path::new(p).is_file());
            if self.store.save_hashes(&self.hashes).is_ok() {
                self.hashes_dirty = false;
            }
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
            _ => return (Status::Missing, t!("guard.file_not_in_this").into(), None),
        };
        if let Ok(age) = meta.modified().and_then(|t| t.elapsed().map_err(io::Error::other)) {
            if age < self.settle {
                return (Status::Busy, t!("guard.being_written_checking_again").into(), None);
            }
        }
        let sha = match self.hash(&path, &meta) {
            Ok(s) => s,
            Err(e) => return (Status::Busy, tf!("guard.cannot_be_read_yet", e = e), None),
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
                        return (Status::Original, t!("guard.game_s_original_file").into(), None);
                    }
                    if self.cached(&l.patched_sha256).is_some() {
                        return match self.install(&path, &sha, l) {
                            Ok(()) => (
                                Status::Patched,
                                t!("guard.patch_reinstalled").into(),
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
                    (Status::Patched, t!("guard.older_build").into(), None)
                } else {
                    (Status::Unknown, t!("guard.not_checked_guard_paused").into(), None)
                };
            }
        }

        let r = match self.remote_entry(profile, name, UNKNOWN_TTL) {
            Ok(Some(r)) => r,
            Ok(None) => {
                return (Status::Unknown, t!("guard.the_latest_release_does").into(), None);
            }
            Err(e) => return (Status::Unknown, tf!("guard.cannot_check_the_release", e = e), None),
        };
        if sha == r.patched_sha256 {
            // Installed by hand (or by an earlier run): adopt it and keep a cached copy.
            if self.cached(&r.patched_sha256).is_none() {
                let _ = copy_atomic(&path, &self.store.patched_path(&r.patched_sha256));
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
                    t!("guard.patch_installed").into(),
                    Some(Event::Installed { profile, name: name.to_string() }),
                ),
                Err(f) => f,
            };
        }
        if self.state.installed.get(&pkey) == Some(&sha) {
            return (
                Status::Patched,
                t!("guard.older_build_the_game").into(),
                None,
            );
        }
        (
            Status::Unknown,
            t!("guard.file_version_does_not").into(),
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
                t!("guard.translation_updated").into(),
                Some(Event::Installed { profile, name: local.name.clone() }),
            ),
            Err((_, why, _)) => (Status::Patched, tf!("guard.translation_update_failed", why = why), None),
        }
    }

    fn fetch_and_install(
        &mut self,
        profile: Profile,
        path: &Path,
        cur_sha: &str,
        r: &FileEntry,
    ) -> Result<(), Outcome> {
        if self.cached(&r.patched_sha256).is_none() {
            let cache = self.store.patched_path(&r.patched_sha256);
            let part = self.store.download_path(&r.patched_sha256);
            let got = self
                .fetch_archive(profile)
                .and_then(|zip| extract(&zip, &r.name, r.patched_size, &part))
                .and_then(|()| sha256_file(&part).map_err(|e| e.to_string()))
                .and_then(|sha| {
                    if sha == r.patched_sha256 {
                        fs::rename(&part, &cache).map_err(|e| e.to_string())
                    } else {
                        Err(t!("guard.zip_content_does_not").to_string())
                    }
                });
            if let Err(e) = got {
                let _ = fs::remove_file(&part);
                self.store.clear_archives();
                self.manifests.remove(&profile);
                return Err((Status::Error, tf!("guard.download_failed", e = e), None));
            }
        }
        self.install(path, cur_sha, r)?;
        self.state.entries.insert(entry_key(profile, &r.name), r.clone());
        Ok(())
    }

    /// The release zip of the cached manifest, downloaded once per check pass.
    fn fetch_archive(&mut self, profile: Profile) -> Result<PathBuf, String> {
        let archive = match self.manifests.get(&profile) {
            Some((_, Ok(m), _)) => m.archive.clone(),
            _ => return Err(t!("guard.release_manifest_not_loaded").into()),
        };
        let zip = self.store.archive_path(&archive.sha256);
        if zip.is_file() {
            return Ok(zip);
        }
        if let Some(dir) = zip.parent() {
            fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        let part = zip.with_extension("part");
        let got = self
            .remote
            .download(profile, &archive, &part)
            .and_then(|()| sha256_file(&part).map_err(|e| e.to_string()))
            .and_then(|sha| {
                if sha == archive.sha256 {
                    fs::rename(&part, &zip).map_err(|e| e.to_string())
                } else {
                    // The rolling release can be replaced mid-download: just retry later.
                    Err(t!("guard.download_does_not_match").to_string())
                }
            });
        if got.is_err() {
            let _ = fs::remove_file(&part);
        }
        got.map(|()| zip)
    }

    /// Replace `path` (currently `cur_sha`) with the cached patched build of `entry`.
    fn install(&mut self, path: &Path, cur_sha: &str, entry: &FileEntry) -> Result<(), Outcome> {
        // Held open by the game or the patcher: the final rename would fail anyway, so do not
        // copy 50-85 MB just to throw it away (that happened every 5 s while the game ran).
        if path.is_file() && !replaceable(path) {
            self.locked_now = true;
            return Err((Status::Busy, t!("guard.file_is_in_use").into(), None));
        }
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
        // Last check before the game folder is touched: never install a damaged cache. The
        // memoized digest is dropped first, so this really re-reads it.
        self.forget_hash(&self.store.patched_path(&entry.patched_sha256));
        let Some(src) = self.cached(&entry.patched_sha256) else {
            return Err((
                Status::Error,
                t!("guard.cached_patch_copy_is").into(),
                None,
            ));
        };
        if let Err(e) = copy_atomic(&src, path) {
            self.locked_now = true;
            return Err(write_failure(e));
        }
        // What was just written is the verified cache copy: remember its hash, so the pass the
        // watcher triggers for our own write does not read 85 MB again.
        self.remember_hash(path, &entry.patched_sha256);
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
        let fails = self.manifest_failures.get(&profile).copied().unwrap_or(0);
        let error_ttl = self
            .error_ttl
            .saturating_mul(1 << fails.saturating_sub(1).min(16))
            .min(ERROR_TTL_MAX.max(self.error_ttl));
        let lang = i18n::get();
        let fresh = self.manifests.get(&profile).is_some_and(|(at, m, phrased)| match m {
            Ok(_) => at.elapsed() < max_age,
            Err(_) => *phrased == lang && at.elapsed() < max_age.min(error_ttl),
        });
        if !fresh {
            let m = self.remote.manifest(profile).and_then(|m| self.not_older(profile, m));
            if m.is_ok() {
                self.manifest_failures.remove(&profile);
            } else {
                *self.manifest_failures.entry(profile).or_insert(0) += 1;
            }
            self.manifests.insert(profile, (Instant::now(), m, lang));
        }
        match &self.manifests[&profile].1 {
            Ok(m) => Ok(m.entry(name).cloned()),
            Err(e) => Err(e.clone()),
        }
    }

    /// Refuse a manifest signed before the newest one this profile already used (an old
    /// signed release served again); remember a newer one.
    fn not_older(&mut self, profile: Profile, m: Manifest) -> Result<Manifest, String> {
        let seen = self.state.manifest_signed_at.entry(profile.key().to_string()).or_insert(0);
        if m.signed_at < *seen {
            return Err(t!("guard.manifest_is_older").into());
        }
        if m.signed_at > *seen {
            // The checkpoint must be on disk before anything from this manifest is installed,
            // or a restart could accept an older one again. When it cannot be written the
            // manifest is refused (and retried on the next pass, like any failed fetch):
            // a disk that cannot take a small state.json cannot take an 85 MB install either,
            // so nothing that would have worked is lost.
            let old = std::mem::replace(seen, m.signed_at);
            if let Err(e) = self.store.save_state(&self.state) {
                self.state.manifest_signed_at.insert(profile.key().to_string(), old);
                return Err(tf!("guard.cannot_save_state", e = e));
            }
        }
        Ok(m)
    }

    /// The cached patched build `sha`, if it is there and still has that hash. A damaged copy
    /// (disk error, antivirus) is deleted so the next install downloads it again.
    fn cached(&mut self, sha: &str) -> Option<PathBuf> {
        let p = self.store.patched_path(sha);
        let meta = fs::metadata(&p).ok().filter(|m| m.is_file())?;
        match self.hash(&p, &meta) {
            Ok(h) if h == sha => Some(p),
            Ok(_) => {
                let _ = fs::remove_file(&p);
                self.forget_hash(&p);
                None
            }
            Err(_) => None,
        }
    }

    /// sha256 of `path`, recomputed only when its size, write time or creation time changed
    /// (the files are 50-85 MB and this runs on every poll). A copy keeps the write time of
    /// its source, so the creation time is what tells a replaced file apart.
    fn hash(&mut self, path: &Path, meta: &fs::Metadata) -> io::Result<String> {
        let key = path_key(path);
        let (len, modified, created) = stamp(meta);
        // Without a write time there is nothing reliable to compare: always re-read.
        if let Some(e) = self.hashes.get(&key)
            && modified.is_some()
            && (e.len, e.modified, e.created) == (len, modified, created)
        {
            return Ok(e.sha256.clone());
        }
        let sha = sha256_file(path)?;
        self.hashes.insert(key, HashEntry { len, modified, created, sha256: sha.clone() });
        self.hashes_dirty = true;
        Ok(sha)
    }

    fn forget_hash(&mut self, path: &Path) {
        if self.hashes.remove(&path_key(path)).is_some() {
            self.hashes_dirty = true;
        }
    }

    /// `path` was just written with content of hash `sha`.
    fn remember_hash(&mut self, path: &Path, sha: &str) {
        match fs::metadata(path) {
            Ok(meta) => {
                let (len, modified, created) = stamp(&meta);
                let e = HashEntry { len, modified, created, sha256: sha.to_string() };
                self.hashes.insert(path_key(path), e);
                self.hashes_dirty = true;
            }
            Err(_) => self.forget_hash(path),
        }
    }
}

fn stamp(meta: &fs::Metadata) -> (u64, Option<i128>, Option<i128>) {
    (meta.len(), meta.modified().ok().map(signed_nanos), meta.created().ok().map(signed_nanos))
}

/// Nanoseconds since the Unix epoch, negative before it, so every distinct time stays distinct.
fn signed_nanos(t: SystemTime) -> i128 {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_nanos() as i128,
        Err(e) => -(e.duration().as_nanos() as i128),
    }
}

/// Whether `path` can be replaced right now: opened for delete access with full sharing, which
/// fails while another process holds it open without delete sharing (the game, the patcher).
fn replaceable(path: &Path) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const DELETE: u32 = 0x0001_0000;
        const SHARE_ALL: u32 = 0x1 | 0x2 | 0x4;
        fs::OpenOptions::new().access_mode(DELETE).share_mode(SHARE_ALL).open(path).is_ok()
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        true
    }
}

fn path_key(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// A failed write is almost always the game or the patcher holding the file: retry.
fn write_failure(e: io::Error) -> Outcome {
    (Status::Busy, tf!("guard.cannot_be_written_yet", e = e), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Archive, MANIFEST_SCHEMA};
    use crate::remote::zip_of;
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
        /// The signing time the next manifest carries.
        signed_at: Cell<u64>,
    }

    impl Fake {
        fn publish(&self, original: &[u8], patched: &[u8]) {
            *self.build.borrow_mut() = Some((original.to_vec(), patched.to_vec()));
            *self.served.borrow_mut() = Some(zip_of(&[(NAME, patched)]));
        }
    }

    impl Remote for Fake {
        fn manifest(&self, _: Profile) -> Result<Manifest, String> {
            self.fetches.set(self.fetches.get() + 1);
            let b = self.build.borrow();
            let (o, p) = b.as_ref().ok_or(t!("common.release_no_manifest"))?;
            let zip = zip_of(&[(NAME, p)]);
            Ok(Manifest {
                schema: MANIFEST_SCHEMA,
                profile: "global".into(),
                commit: String::new(),
                built_at: String::new(),
                archive: Archive {
                    name: "wwm-indonesian-patch.zip".into(),
                    sha256: sha(&zip),
                    size: zip.len() as u64,
                },
                files: vec![FileEntry {
                    name: NAME.into(),
                    original_sha256: sha(o),
                    original_size: o.len() as u64,
                    patched_sha256: sha(p),
                    patched_size: p.len() as u64,
                }],
                signed_at: self.signed_at.get(),
            })
        }

        fn download(&self, _: Profile, _: &Archive, dest: &Path) -> Result<(), String> {
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
    fn a_newer_manifest_is_not_used_until_its_checkpoint_is_saved() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.signed_at.set(2000);
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        // state.json cannot be written: a folder sits where the file goes.
        let state = r.guard.store.base().join("state.json");
        fs::create_dir_all(&state).unwrap();
        assert!(r.guard.remote_entry(Profile::Global, NAME, Duration::ZERO).is_err());
        assert_eq!(r.content(), b"orig-v1", "nothing installed from it");
        assert_eq!(r.guard.state.manifest_signed_at.get("global"), Some(&0));
        // Once the disk takes it, the same manifest is used.
        fs::remove_dir(&state).unwrap();
        assert!(r.guard.remote_entry(Profile::Global, NAME, Duration::ZERO).is_ok());
        assert!(r.guard.store.load_state().manifest_signed_at.get("global") == Some(&2000));
    }

    #[test]
    fn an_older_signed_manifest_is_refused() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.signed_at.set(2000);
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        assert_eq!(r.check().0, Status::Patched);
        // The release is swapped for an older signed build with another translation.
        r.guard.remote.signed_at.set(1000);
        r.guard.remote.publish(b"orig-v1", b"patched-v0");
        r.check();
        assert_eq!(r.content(), b"patched-v1", "no rollback");
        // Even after a restart (the time is kept in state.json).
        let store = r.guard.store.clone();
        let mut g = Guard::new(store, Fake::default());
        g.remote.signed_at.set(1000);
        g.remote.publish(b"orig-v1", b"patched-v0");
        assert!(g.remote_entry(Profile::Global, NAME, Duration::ZERO).is_err());
        // A newer one is fine.
        g.remote.signed_at.set(3000);
        assert!(g.remote_entry(Profile::Global, NAME, Duration::ZERO).is_ok());
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
        *r.guard.remote.served.borrow_mut() = Some(zip_of(&[(NAME, b"something else")]));
        assert_eq!(r.check(), (Status::Error, vec![]));
        assert_eq!(r.content(), b"orig-v1");
        // Fixed on the server: the next pass succeeds.
        *r.guard.remote.served.borrow_mut() = Some(zip_of(&[(NAME, b"patched-v1")]));
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
    }

    #[test]
    fn failed_manifest_fetch_is_retried_soon() {
        let mut r = Rig::new(b"orig-v1");
        // Offline: the first pass fails to fetch the manifest.
        assert_eq!(r.check_with(Opts::default(), false).0, Status::Unknown);
        assert_eq!(r.guard.remote.fetches.get(), 1);
        // Within the error TTL nothing is fetched again…
        assert_eq!(r.check_with(Opts::default(), false).0, Status::Unknown);
        assert_eq!(r.guard.remote.fetches.get(), 1);
        // …after it, the next pass tries again and installs (not 10 min / 6 h later).
        r.guard.error_ttl = Duration::ZERO;
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        assert_eq!(r.check_with(Opts::default(), false), (Status::Patched, vec![installed()]));
        assert_eq!(r.guard.remote.fetches.get(), 2);
    }

    #[test]
    fn damaged_cache_is_downloaded_again_not_installed() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        let cache = r.guard.store.patched_path(&sha(b"patched-v1"));
        fs::write(&cache, b"bit rot").unwrap();
        r.overwrite(b"orig-v1");
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v1");
        assert_eq!(r.guard.remote.downloads.get(), 2, "the damaged copy was fetched again");
        assert_eq!(fs::read(&cache).unwrap(), b"patched-v1");
    }

    #[test]
    fn cache_damaged_without_a_size_or_mtime_change_is_caught_at_install() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        // Same length, same mtime: the memoized digest alone would still call it good.
        let cache = r.guard.store.patched_path(&sha(b"patched-v1"));
        let mtime = fs::metadata(&cache).unwrap().modified().unwrap();
        fs::write(&cache, b"PATCHED-V1").unwrap();
        fs::File::options().write(true).open(&cache).unwrap().set_modified(mtime).unwrap();
        r.overwrite(b"orig-v1");
        assert_eq!(r.check_with(Opts::default(), false).0, Status::Error);
        assert_eq!(r.content(), b"orig-v1", "the damaged copy is not installed");
        assert!(!cache.exists(), "and it is deleted");
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v1");
    }

    #[test]
    fn a_failed_fetch_is_not_reused_after_a_language_switch() {
        let mut r = Rig::new(b"orig-v1");
        i18n::set(Lang::Id);
        r.check_with(Opts::default(), false);
        assert_eq!(r.guard.remote.fetches.get(), 1);
        i18n::set(Lang::En);
        r.check_with(Opts::default(), false);
        assert_eq!(r.guard.remote.fetches.get(), 2, "fetched again, phrased in English");
        r.check_with(Opts::default(), false);
        assert_eq!(r.guard.remote.fetches.get(), 2, "same language: the error TTL holds");
        i18n::set(Lang::Id);
    }

    #[test]
    fn release_zip_is_not_kept_after_the_pass() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        let archives = r._tmp.path().join("data").join("archives");
        assert!(!archives.exists(), "zip left behind in {}", archives.display());
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
        // Nothing was copied into the game folder just to fail (no temp file left behind).
        let dir = r.file.parent().unwrap().to_path_buf();
        let tmp = fs::read_dir(&dir).unwrap().flatten().any(|e| e.file_name().to_string_lossy().ends_with(".wwmpg-tmp"));
        assert!(!tmp);
        // While it stays locked the retry backs off: 5 s, 10 s, 20 s…
        assert_eq!(r.guard.locked_retry(), Some(Duration::from_secs(5)));
        r.check();
        assert_eq!(r.guard.locked_retry(), Some(Duration::from_secs(10)));
        r.check();
        assert_eq!(r.guard.locked_retry(), Some(Duration::from_secs(20)));
        drop(held);
        assert_eq!(r.check(), (Status::Patched, vec![installed()]));
        assert_eq!(r.content(), b"patched-v1");
        assert_eq!(r.guard.locked_retry(), None, "back to the normal poll");
    }

    #[test]
    fn times_before_1970_stay_distinct() {
        let before = |s| UNIX_EPOCH - Duration::from_secs(s);
        assert_ne!(signed_nanos(before(1)), signed_nanos(before(2)));
        assert_eq!(signed_nanos(before(1)), -1_000_000_000);
        assert_eq!(signed_nanos(UNIX_EPOCH + Duration::from_secs(1)), 1_000_000_000);
    }

    #[test]
    fn hashes_survive_a_restart() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.check();
        let key = r.file.to_string_lossy().into_owned();
        let saved = r.guard.store.load_hashes();
        assert_eq!(saved.get(&key).map(|e| e.sha256.as_str()), Some(sha(b"patched-v1").as_str()));
        // A fresh guard (the next start) takes the stored hash instead of reading the file: a
        // planted entry for the current size and times is believed as is.
        let mut fake = saved.clone();
        fake.get_mut(&key).unwrap().sha256 = "0".repeat(64);
        r.guard.store.save_hashes(&fake).unwrap();
        let mut g = Guard::new(r.guard.store.clone(), Fake::default());
        let meta = fs::metadata(&r.file).unwrap();
        assert_eq!(g.hash(&r.file, &meta).unwrap(), "0".repeat(64));
        // Entries for files that are gone are dropped when saved.
        g.hashes.insert(r.file.with_file_name("gone").to_string_lossy().into_owned(), saved[&key].clone());
        g.hashes_dirty = true;
        g.flush();
        assert_eq!(r.guard.store.load_hashes().len(), saved.len());
    }

    #[test]
    fn an_offline_release_is_retried_less_and_less_often() {
        let mut r = Rig::new(b"orig-v1");
        r.guard.error_ttl = Duration::from_secs(60);
        for _ in 0..4 {
            // No manifest: every forced fetch fails.
            *r.guard.remote.build.borrow_mut() = None;
            let _ = r.guard.remote_entry(Profile::Global, NAME, Duration::from_secs(3600));
            r.guard.manifests.get_mut(&Profile::Global).unwrap().0 = Instant::now() - Duration::from_secs(3600);
        }
        assert_eq!(r.guard.manifest_failures[&Profile::Global], 4);
        // After 4 failures the next attempt waits 8 min (60 s doubled three times)…
        r.guard.manifests.get_mut(&Profile::Global).unwrap().0 = Instant::now() - Duration::from_secs(7 * 60);
        let fetches = r.guard.remote.fetches.get();
        let _ = r.guard.remote_entry(Profile::Global, NAME, Duration::from_secs(3600));
        assert_eq!(r.guard.remote.fetches.get(), fetches, "not yet");
        // …and a success resets it.
        r.guard.remote.publish(b"orig-v1", b"patched-v1");
        r.guard.manifests.get_mut(&Profile::Global).unwrap().0 = Instant::now() - Duration::from_secs(9 * 60);
        assert!(r.guard.remote_entry(Profile::Global, NAME, Duration::from_secs(3600)).is_ok());
        assert!(!r.guard.manifest_failures.contains_key(&Profile::Global));
    }
}
