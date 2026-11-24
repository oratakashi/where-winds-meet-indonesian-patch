//! On-disk state under `%LOCALAPPDATA%\com.oratakashi.wwm\` — never inside the game folder,
//! because NetEase's patcher deletes files it does not know there.
//!
//! ```text
//! config.json      folders to guard + switches
//! history.json     what the guard did (shown on the History page)
//! state.json       which build is cached per file, which original each path had
//! hashes.json      sha256 per file + the size/times it was computed for (skip re-hashing)
//! patched/<sha>    cached patched builds, content-addressed
//! originals/<sha>  files the guard replaced, content-addressed (for "restore original")
//! updates/         a downloaded, signed Patch Guard update (see `update.rs`)
//! ```
use crate::i18n::Lang;
use crate::model::{FileEntry, LogLine, Profile};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct GuardDir {
    pub profile: Profile,
    pub path: PathBuf,
}

fn yes() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default)]
    pub dirs: Vec<GuardDir>,
    /// Install a newer translation build when one is published for the same game files.
    #[serde(default = "yes")]
    pub auto_update: bool,
    #[serde(default)]
    pub paused: bool,
    /// Download new Patch Guard versions in the background and offer to restart.
    #[serde(default = "yes")]
    pub auto_update_app: bool,
    /// Install a downloaded update on quit / the next start without asking. Off by default:
    /// otherwise only "Restart sekarang" (window or tray) installs it.
    #[serde(default)]
    pub auto_install_app: bool,
    /// UI language; a config from before v0.6 has none and gets Indonesian.
    #[serde(default)]
    pub language: Lang,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dirs: Vec::new(),
            auto_update: true,
            paused: false,
            auto_update_app: true,
            auto_install_app: false,
            language: Lang::Id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct State {
    /// `"<profile>/<file name>"` -> the build whose patched file sits in `patched/`.
    #[serde(default)]
    pub entries: BTreeMap<String, FileEntry>,
    /// Full path of a guarded file -> sha256 of the last non-patched content replaced there.
    #[serde(default)]
    pub backups: BTreeMap<String, String>,
    /// Full path of a guarded file -> sha256 of the patched build last put (or found) there.
    /// Tells "an older build of ours" apart from a file the guard knows nothing about.
    #[serde(default)]
    pub installed: BTreeMap<String, String>,
    /// Profile key -> signing time (unix secs) of the newest release manifest used. Never
    /// goes down, so an older signed manifest is refused (anti-rollback).
    #[serde(default)]
    pub manifest_signed_at: BTreeMap<String, u64>,
}

/// A file's sha256 as last computed, and the metadata it was computed for. Kept in
/// `hashes.json`, so a start (autostart at login) does not re-read every 50–85 MB file.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct HashEntry {
    pub len: u64,
    /// Last write / creation time, nanoseconds since the Unix epoch (negative before it). The
    /// creation time changes when a file is replaced by a rename, even if the size and write
    /// time stay the same.
    pub modified: Option<i128>,
    pub created: Option<i128>,
    pub sha256: String,
}

/// Full path -> [`HashEntry`].
pub type HashCache = BTreeMap<String, HashEntry>;

pub fn entry_key(profile: Profile, name: &str) -> String {
    format!("{}/{}", profile.key(), name)
}

#[derive(Clone, Debug)]
pub struct Store {
    base: PathBuf,
}

impl Store {
    pub fn new(base: impl Into<PathBuf>) -> io::Result<Store> {
        let base = base.into();
        fs::create_dir_all(base.join("patched"))?;
        fs::create_dir_all(base.join("originals"))?;
        Ok(Store { base })
    }

    /// The data folder itself.
    pub fn base(&self) -> &Path {
        &self.base
    }

    pub fn patched_path(&self, sha: &str) -> PathBuf {
        self.base.join("patched").join(sha)
    }

    pub fn original_path(&self, sha: &str) -> PathBuf {
        self.base.join("originals").join(sha)
    }

    /// Scratch path for a download in progress.
    pub fn download_path(&self, sha: &str) -> PathBuf {
        self.base.join("patched").join(format!("{sha}.part"))
    }

    /// Downloaded release zip, kept only for the check pass that needed it.
    pub fn archive_path(&self, sha: &str) -> PathBuf {
        self.base.join("archives").join(format!("{sha}.zip"))
    }

    /// Drop every downloaded release zip: what they held is in `patched/` by now.
    pub fn clear_archives(&self) {
        let _ = fs::remove_dir_all(self.base.join("archives"));
    }

    pub fn load_config(&self) -> Config {
        self.load("config.json")
    }

    /// Like [`load_config`](Self::load_config), plus `Some` when `config.json` exists but
    /// cannot be parsed (the defaults are used until the next save): where its copy was kept,
    /// or why the copy could not be written.
    pub fn load_config_checked(&self) -> (Config, Option<io::Result<PathBuf>>) {
        self.load_checked("config.json")
    }

    pub fn save_config(&self, c: &Config) -> io::Result<()> {
        self.save("config.json", c)
    }

    /// Entries whose hashes are not plain sha256 are dropped: those hashes are used as file
    /// names under `patched/` and `originals/`.
    pub fn load_state(&self) -> State {
        let mut s: State = self.load("state.json");
        s.entries.retain(|_, e| is_sha256_hex(&e.original_sha256) && is_sha256_hex(&e.patched_sha256));
        s.backups.retain(|_, sha| is_sha256_hex(sha));
        s.installed.retain(|_, sha| is_sha256_hex(sha));
        s
    }

    pub fn save_state(&self, s: &State) -> io::Result<()> {
        self.save("state.json", s)
    }

    /// Entries with a hash that is not plain sha256 are dropped.
    pub fn load_hashes(&self) -> HashCache {
        let mut h: HashCache = self.load("hashes.json");
        h.retain(|_, e| is_sha256_hex(&e.sha256));
        h
    }

    pub fn save_hashes(&self, h: &HashCache) -> io::Result<()> {
        self.save("hashes.json", h)
    }

    pub fn load_history(&self) -> Vec<LogLine> {
        self.load("history.json")
    }

    pub fn save_history(&self, log: &[LogLine]) -> io::Result<()> {
        self.save("history.json", &log)
    }

    /// Drop cached files nothing in `state` refers to any more.
    pub fn prune(&self, state: &State) {
        let keep_patched: Vec<&str> =
            state.entries.values().map(|e| e.patched_sha256.as_str()).collect();
        let keep_orig: Vec<&str> = state.backups.values().map(String::as_str).collect();
        for (dir, keep) in [("patched", keep_patched), ("originals", keep_orig)] {
            let Ok(rd) = fs::read_dir(self.base.join(dir)) else { continue };
            for f in rd.flatten() {
                let name = f.file_name().to_string_lossy().into_owned();
                if !keep.contains(&name.as_str()) {
                    let _ = fs::remove_file(f.path());
                }
            }
        }
    }

    fn load<T: DeserializeOwned + Default>(&self, name: &str) -> T {
        self.load_checked(name).0
    }

    /// A missing file gives the defaults. A file that exists but does not parse gives them
    /// too, and is first copied to `<name>.bad`: the next save replaces it, and what it held
    /// must not be lost without a trace. Then the second value is `Some` — the copy's path, or
    /// the error that kept it from being written (the broken file is reported either way).
    fn load_checked<T: DeserializeOwned + Default>(
        &self,
        name: &str,
    ) -> (T, Option<io::Result<PathBuf>>) {
        let path = self.base.join(name);
        let Ok(bytes) = fs::read(&path) else { return (T::default(), None) };
        match serde_json::from_slice(&bytes) {
            Ok(v) => (v, None),
            Err(_) => {
                let bad = self.base.join(format!("{name}.bad"));
                (T::default(), Some(fs::write(&bad, &bytes).map(|()| bad)))
            }
        }
    }

    /// Write through a temp file that is flushed to disk before the rename, so a power cut
    /// leaves the old file or the new one, never an empty one.
    fn save<T: Serialize>(&self, name: &str, v: &T) -> io::Result<()> {
        let dest = self.base.join(name);
        let tmp = self.base.join(format!("{name}.tmp"));
        let data = serde_json::to_vec_pretty(v).map_err(io::Error::other)?;
        let mut f = fs::File::create(&tmp)?;
        f.write_all(&data)?;
        f.sync_all()?;
        drop(f);
        fs::rename(&tmp, &dest)
    }
}

pub fn sha256_file(path: &Path) -> io::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex(&h.finalize()))
}

/// sha256 of bytes already in memory, as lowercase hex.
pub fn sha256_bytes(data: &[u8]) -> String {
    hex(&Sha256::digest(data))
}

fn hex(digest: &[u8]) -> String {
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// A sha256 as this app writes it: 64 lowercase hex digits. Hashes from the manifest and from
/// `state.json` become file names under the data folder, so nothing else is accepted.
pub fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// The temp file [`copy_atomic`] writes next to `dest`: `.<name>.wwmpg-tmp`.
pub fn temp_path(dest: &Path) -> PathBuf {
    let name = dest.file_name().unwrap_or_default().to_string_lossy();
    dest.with_file_name(format!(".{name}.wwmpg-tmp"))
}

/// Copy `src` over `dest` through a temp file in `dest`'s folder, so a reader never sees a
/// half-written file. Fails (leaving `dest` untouched) while another process holds `dest`
/// open without delete sharing — the caller retries later.
pub fn copy_atomic(src: &Path, dest: &Path) -> io::Result<()> {
    let tmp = temp_path(dest);
    let res = fs::copy(src, &tmp).and_then(|_| fs::rename(&tmp, dest));
    if res.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    res.map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_without_language_is_indonesian_and_english_is_kept() {
        let c: Config = serde_json::from_str(r#"{"dirs": [], "paused": false}"#).unwrap();
        assert_eq!(c.language, Lang::Id);
        assert!(c.auto_update && c.auto_update_app);
        assert!(!c.auto_install_app, "installing without asking is opt-in");
        let tmp = tempfile::tempdir().unwrap();
        let s = Store::new(tmp.path()).unwrap();
        s.save_config(&Config { language: Lang::En, ..Config::default() }).unwrap();
        assert_eq!(s.load_config().language, Lang::En);
    }

    #[test]
    fn a_broken_config_is_kept_aside_not_lost() {
        let tmp = tempfile::tempdir().unwrap();
        let s = Store::new(tmp.path()).unwrap();
        assert!(s.load_config_checked().1.is_none(), "no file is not an error");
        fs::write(tmp.path().join("config.json"), b"{\"dirs\": [ broken").unwrap();
        let (c, bad) = s.load_config_checked();
        assert!(c.dirs.is_empty());
        let bad = bad.expect("broken config reported").expect("copy written");
        assert_eq!(fs::read(&bad).unwrap(), b"{\"dirs\": [ broken");
        // Saving the defaults replaces config.json; the copy stays.
        s.save_config(&c).unwrap();
        assert!(s.load_config_checked().1.is_none());
        assert!(bad.is_file());
        assert!(!tmp.path().join("config.json.tmp").exists());
    }

    #[test]
    fn a_broken_config_is_reported_even_when_its_copy_fails() {
        let tmp = tempfile::tempdir().unwrap();
        let s = Store::new(tmp.path()).unwrap();
        fs::write(tmp.path().join("config.json"), b"not json").unwrap();
        // A folder where the copy would go: it cannot be written.
        fs::create_dir(tmp.path().join("config.json.bad")).unwrap();
        let (_, bad) = s.load_config_checked();
        assert!(bad.expect("broken config reported").is_err());
    }
}
