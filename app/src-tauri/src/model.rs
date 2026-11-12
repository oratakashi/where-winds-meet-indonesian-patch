//! Types shared by every backend module, plus the release manifest written by
//! `tools/make_manifest.py`.
use serde::{Deserialize, Serialize};

pub const MANIFEST_SCHEMA: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    Global,
    Cn,
}

impl Profile {
    pub const ALL: [Profile; 2] = [Profile::Global, Profile::Cn];

    pub fn key(self) -> &'static str {
        match self {
            Profile::Global => "global",
            Profile::Cn => "cn",
        }
    }

    /// Rolling GitHub release that carries this profile's build.
    pub fn release_tag(self) -> &'static str {
        match self {
            Profile::Global => "latest",
            Profile::Cn => "cn-latest",
        }
    }

    /// Files the patch replaces. The CN `*_diff` files are left alone (see README).
    pub fn files(self) -> &'static [&'static str] {
        match self {
            Profile::Global => &["translate_words_map_en", "translate_words_map_en_diff"],
            Profile::Cn => &["translate_words_map_zh_cn", "translate_words_map_zh_cn__small"],
        }
    }

    /// Locale folders relative to a game variant folder (e.g. `wwm_lite`, `yysls_fast`).
    pub fn locale_dirs(self) -> &'static [&'static str] {
        match self {
            Profile::Global => &[
                r"LocalData\Patch\HD\oversea\locale",
                r"Package\HD\oversea\locale",
            ],
            Profile::Cn => &[r"LocalData\Patch\HD\v2\locale"],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct FileEntry {
    pub name: String,
    pub original_sha256: String,
    pub original_size: u64,
    pub patched_sha256: String,
    pub patched_size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    pub schema: u32,
    pub profile: String,
    #[serde(default)]
    pub commit: String,
    #[serde(default)]
    pub built_at: String,
    pub files: Vec<FileEntry>,
}

impl Manifest {
    pub fn entry(&self, name: &str) -> Option<&FileEntry> {
        self.files.iter().find(|f| f.name == name)
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Installed file is the known patched build.
    Patched,
    /// Installed file is the game's original and the guard did not touch it (paused).
    Original,
    /// Installed file matches no build we know: left untouched.
    Unknown,
    Missing,
    /// Still being written, or locked by the game/patcher. Retried shortly.
    Busy,
    Error,
}

#[derive(Serialize, Clone, Debug)]
pub struct FileReport {
    pub name: String,
    pub status: Status,
    pub detail: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct DirReport {
    pub profile: Profile,
    pub path: String,
    pub files: Vec<FileReport>,
}

/// Something the guard did, worth a log line and a notification.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
    /// The patcher put the original back; the cached patched build was reinstalled.
    Restored { profile: Profile, name: String },
    /// A build was downloaded and installed (new game version or new translation).
    Installed { profile: Profile, name: String },
    /// The original/previous file was put back on request.
    Uninstalled { profile: Profile, name: String },
}
