//! On-disk state under `%LOCALAPPDATA%\io.github.oratakashi.wwm-patch-guard\` — never inside the game folder,
//! because NetEase's patcher deletes files it does not know there.
//!
//! ```text
//! config.json      folders to guard + switches
//! state.json       which build is cached per file, which original each path had
//! patched/<sha>    cached patched builds, content-addressed
//! originals/<sha>  files the guard replaced, content-addressed (for "restore original")
//! ```
use crate::model::{FileEntry, Profile};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read};
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
}

impl Default for Config {
    fn default() -> Self {
        Config { dirs: Vec::new(), auto_update: true, paused: false }
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
}

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

    pub fn load_config(&self) -> Config {
        self.load("config.json")
    }

    pub fn save_config(&self, c: &Config) -> io::Result<()> {
        self.save("config.json", c)
    }

    pub fn load_state(&self) -> State {
        self.load("state.json")
    }

    pub fn save_state(&self, s: &State) -> io::Result<()> {
        self.save("state.json", s)
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
        fs::read(self.base.join(name))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }

    fn save<T: Serialize>(&self, name: &str, v: &T) -> io::Result<()> {
        let dest = self.base.join(name);
        let tmp = self.base.join(format!("{name}.tmp"));
        fs::write(&tmp, serde_json::to_vec_pretty(v).map_err(io::Error::other)?)?;
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
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

/// Copy `src` over `dest` through a temp file in `dest`'s folder, so a reader never sees a
/// half-written file. Fails (leaving `dest` untouched) while another process holds `dest`
/// open without delete sharing — the caller retries later.
pub fn copy_atomic(src: &Path, dest: &Path) -> io::Result<()> {
    let name = dest.file_name().unwrap_or_default().to_string_lossy();
    let tmp = dest.with_file_name(format!(".{name}.wwmpg-tmp"));
    let res = fs::copy(src, &tmp).and_then(|_| fs::rename(&tmp, dest));
    if res.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    res.map(|_| ())
}
