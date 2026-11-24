//! Types shared by every backend module, plus the release manifest written by
//! `tools/make_manifest.py`.
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 2: the release carries one zip (`archive`) instead of every file as its own asset.
pub const MANIFEST_SCHEMA: u32 = 2;

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

    /// Locale folders worth guarding, relative to a game folder (`wwm_lite`, `yysls_fast`,
    /// or the Steam install folder).
    ///
    /// On Steam only `Package` holds: the in-game patcher re-downloads the `LocalData\Patch`
    /// overlay on every launch, so guarding it would only fight the patcher.
    pub fn locale_dirs(self, client: Client) -> &'static [&'static str] {
        match (self, client) {
            (Profile::Global, Client::Steam) => &[PACKAGE_LOCALE],
            (Profile::Global, Client::Launcher) => &[OVERLAY_LOCALE, PACKAGE_LOCALE],
            (Profile::Cn, Client::Launcher) => &[r"LocalData\Patch\HD\v2\locale"],
            (Profile::Cn, Client::Steam) => &[],
        }
    }
}

/// The hot-patch overlay: what the official launcher's client loads.
pub const OVERLAY_LOCALE: &str = r"LocalData\Patch\HD\oversea\locale";
/// The packaged copy: the only one that holds on Steam.
pub const PACKAGE_LOCALE: &str = r"Package\HD\oversea\locale";

/// Which client installed the game. It decides which locale folders are worth guarding.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Client {
    Steam,
    Launcher,
}

impl Client {
    /// Steam games live under `<library>\steamapps\common\`; anything else is a launcher install.
    pub fn of(path: &Path) -> Client {
        let steam = path
            .components()
            .any(|c| c.as_os_str().to_string_lossy().eq_ignore_ascii_case("steamapps"));
        if steam { Client::Steam } else { Client::Launcher }
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

/// The release zip: every patched file of `Manifest::files`, stored flat by name.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Archive {
    pub name: String,
    pub sha256: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    pub schema: u32,
    pub profile: String,
    #[serde(default)]
    pub commit: String,
    #[serde(default)]
    pub built_at: String,
    pub archive: Archive,
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
    pub client: Client,
    pub path: String,
    pub files: Vec<FileReport>,
}

/// One line of the history shown in the window.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogLine {
    /// Unix seconds.
    pub at: u64,
    /// The line as written (in the language of that moment). Shown as is when `msg` is
    /// absent: history from before v0.6, and free-form errors carrying OS/network text.
    pub text: String,
    /// What happened, so the line is shown in the current language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<LogMsg>,
}

impl LogLine {
    pub fn new(at: u64, text: String, msg: Option<LogMsg>) -> LogLine {
        LogLine { at, text, msg }
    }

    /// The line in the current language.
    pub fn display(&self) -> String {
        self.msg.as_ref().map_or_else(|| self.text.clone(), LogMsg::text)
    }
}

/// A history line the app can phrase in any language.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LogMsg {
    Restored { name: String },
    Installed { name: String },
    Uninstalled { name: String },
    AppUpdateReady { version: String },
    AppUpdated { version: String },
    AppUpdateFailed { version: String, code: Option<i32> },
}

impl LogMsg {
    pub fn text(&self) -> String {
        match self {
            LogMsg::Restored { name } => tf!("log.overwritten_by_the_patcher", name = name),
            LogMsg::Installed { name } => {
                tf!("log.latest_build_installed", name = name)
            }
            LogMsg::Uninstalled { name } => {
                tf!("log.original_file_restored", name = name)
            }
            LogMsg::AppUpdateReady { version } => tf!("log.patch_guard_v_downloaded", version = version),
            LogMsg::AppUpdated { version } => {
                tf!("log.patch_guard_updated_to", version = version)
            }
            LogMsg::AppUpdateFailed { version, code } => {
                let code = code.map_or_else(|| t!("log.code_did_not_start").to_string(), |c| c.to_string());
                tf!("log.updating_patch_guard_to", version = version, code = code)
            }
        }
    }
}

impl From<&Event> for LogMsg {
    fn from(e: &Event) -> LogMsg {
        match e {
            Event::Restored { name, .. } => LogMsg::Restored { name: name.clone() },
            Event::Installed { name, .. } => LogMsg::Installed { name: name.clone() },
            Event::Uninstalled { name, .. } => LogMsg::Uninstalled { name: name.clone() },
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{self, Lang};

    #[test]
    fn old_history_lines_still_load_and_show_their_text() {
        let l: LogLine = serde_json::from_str(r#"{"at": 5, "text": "file asli dikembalikan"}"#).unwrap();
        assert_eq!((l.msg.as_ref(), l.display().as_str()), (None, "file asli dikembalikan"));
    }

    #[test]
    fn structured_lines_are_shown_in_the_current_language() {
        let m = LogMsg::Restored { name: "translate_words_map_en".into() };
        let l = LogLine::new(1, m.text(), Some(m));
        let json = serde_json::to_string(&l).unwrap();
        assert!(json.contains(r#""kind":"restored""#), "{json}");
        let back: LogLine = serde_json::from_str(&json).unwrap();
        i18n::set(Lang::En);
        assert!(back.display().contains("reinstalled"), "{}", back.display());
        i18n::set(Lang::Id);
        assert!(back.display().contains("dipasang kembali"));
    }
}
