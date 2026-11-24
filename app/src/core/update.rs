//! Self-update, VS Code style: the background thread downloads a newer Patch Guard from the
//! `latest` release and verifies it; the window then offers "Restart sekarang". The update is
//! also installed when the app quits from the tray, or on the next start.
//!
//! Release assets (written by `release-patch.yml`, all signed with the same minisign key as
//! `manifest.json`):
//!
//! ```text
//! patch-guard-update.json(.sig)   {version, name, sha256, size, signature, notes, pub_date}
//! wwm-patch-guard-setup.exe(.sig) the NSIS installer
//! ```
//!
//! A downloaded update is kept as `updates\<version>\` holding the installer plus the signed
//! `patch-guard-update.json` and its `.sig`, exactly as the release served them. Nothing else
//! describes it: every reader ([`Updates::pending`], the helper) checks both signatures, the
//! size, the sha256 and that the version is newer than the running build, every time.
//!
//! Applying never happens in this process: [`apply`] copies the running exe to a fresh folder
//! `<data dir>\updater\<id>\` and starts that copy as a helper (`--apply-update`), which waits
//! for us to exit, verifies the pending update again, runs the installer silently (`/S`,
//! per-user install = no UAC) while holding it open against changes, and starts the installed
//! Patch Guard again. The helper takes no paths from its command line — it finds the data
//! folder and the installed exe itself — so starting it by hand can only ever install a
//! signed, newer Patch Guard. The copy is needed because the installer replaces the exe the
//! app runs from.
use crate::remote::{HttpRemote, LATEST_TAG};
use crate::store::sha256_bytes;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// The update description on the `latest` release.
pub const UPDATE_JSON: &str = "patch-guard-update.json";
const UPDATE_SIG: &str = "patch-guard-update.json.sig";
/// The minisign public key (cargo-packager format: base64 of the `.pub` text).
const PUBKEY: &str = include_str!("../../signing/patch-guard.pub");
/// Largest installer we accept, whatever the JSON says.
const INSTALLER_MAX: u64 = 200 << 20;
const UPDATE_JSON_MAX: u64 = 64 << 10;
const SIG_MAX: u64 = 4096;

pub const ARG_APPLY: &str = "--apply-update";
const ARG_WAIT_PID: &str = "--wait-pid";
const ARG_RELAUNCH: &str = "--relaunch";
/// Only after `--relaunch`: start the app again in the tray.
const ARG_MINIMIZED: &str = "--minimized";
/// What the last update helper did, next to `updates\` (that folder is deleted afterwards).
const RESULT_JSON: &str = "update-result.json";
/// The version whose installer failed, next to `updates\`: kept until a manual check or a
/// successful update, so automatic checks never download that installer again.
const FAILED_JSON: &str = "update-failed.json";
/// Helper copies live in `<data dir>\updater\<id>\`, one fresh folder each.
const HELPER_DIR: &str = "updater";
const HELPER_EXE: &str = "wwm-patch-guard-updater.exe";
/// Where v0.6.0 and earlier put the helper (`%TEMP%\<prefix><pid>.exe`); still cleaned up.
const LEGACY_HELPER_PREFIX: &str = "wwm-patch-guard-updater-";

// ---------------------------------------------------------------- signatures

/// Check a cargo-packager / minisign signature (`.sig` content) of `data` against the
/// built-in key. Debug builds accept `WWM_PATCH_GUARD_PUBKEY` instead (local testing).
pub fn verify(data: &[u8], sig: &str) -> Result<(), String> {
    #[cfg(debug_assertions)]
    if let Ok(k) = std::env::var("WWM_PATCH_GUARD_PUBKEY") {
        if !k.trim().is_empty() {
            return verify_with(k.trim(), data, sig);
        }
    }
    verify_with(PUBKEY.trim(), data, sig)
}

fn verify_with(pubkey_b64: &str, data: &[u8], sig_b64: &str) -> Result<(), String> {
    let text = |b64: &str| -> Result<String, String> {
        let raw = STANDARD.decode(b64.trim()).map_err(|_| t!("update.not_base64").to_string())?;
        String::from_utf8(raw).map_err(|_| t!("update.not_text").to_string())
    };
    let pk = minisign_verify::PublicKey::decode(&text(pubkey_b64)?)
        .map_err(|e| tf!("update.public_key_is_damaged", e = e))?;
    let sig = minisign_verify::Signature::decode(&text(sig_b64)?)
        .map_err(|e| tf!("update.signature_is_damaged", e = e))?;
    pk.verify(data, &sig, false).map_err(|_| t!("update.signature_does_not_match").to_string())
}

/// How a signature is checked: [`verify`] in the app, a test key in the tests.
type Verifier<'a> = &'a dyn Fn(&[u8], &str) -> Result<(), String>;

// ---------------------------------------------------------------- versions

/// `major.minor.patch`, an optional leading `v`, anything after `-`/`+` ignored.
pub fn parse_version(v: &str) -> Option<(u64, u64, u64)> {
    let v = v.trim().trim_start_matches('v');
    let core = v.split(['-', '+']).next()?;
    let mut it = core.split('.').map(|p| p.parse::<u64>().ok());
    let r = (it.next()??, it.next()??, it.next()??);
    it.next().is_none().then_some(r)
}

pub fn is_newer(candidate: &str, current: &str) -> bool {
    matches!((parse_version(candidate), parse_version(current)), (Some(a), Some(b)) if a > b)
}

pub const CURRENT: &str = env!("CARGO_PKG_VERSION");

/// A string that is safe as one file name: letters, digits and `.-_+`, no `..`.
fn plain_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && !s.contains("..")
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'+'))
}

// ---------------------------------------------------------------- release data

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateInfo {
    pub version: String,
    /// Installer asset name on the same release.
    pub name: String,
    pub sha256: String,
    pub size: u64,
    /// The installer's `.sig` content.
    pub signature: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub pub_date: String,
}

/// `patch-guard-update.json` as the release served it, with its signature (already checked).
/// The raw bytes are kept so the pending update can be verified again before it is installed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub info: UpdateInfo,
    json: Vec<u8>,
    sig: String,
}

/// Verify and parse `patch-guard-update.json` against its `.sig`.
pub fn parse_info(json: &[u8], sig: &str) -> Result<UpdateInfo, String> {
    parse_info_with(json, sig, &verify)
}

fn parse_info_with(json: &[u8], sig: &str, verify: Verifier) -> Result<UpdateInfo, String> {
    verify(json, sig).map_err(|e| format!("{UPDATE_JSON}: {e}"))?;
    decode_info(json)
}

fn decode_info(json: &[u8]) -> Result<UpdateInfo, String> {
    let info: UpdateInfo =
        serde_json::from_slice(json).map_err(|e| tf!("update.cannot_be_read", UPDATE_JSON = UPDATE_JSON, e = e))?;
    // The version names the download folder, so it must be a plain name too.
    if parse_version(&info.version).is_none() || !plain_name(&info.version) {
        let version = &info.version;
        return Err(tf!("update.invalid_version", UPDATE_JSON = UPDATE_JSON, version = version));
    }
    // A name, not a path: the installer is always fetched from the same release.
    let exe = info.name.to_ascii_lowercase().ends_with(".exe");
    if !plain_name(&info.name) || !exe {
        return Err(tf!("update.invalid_installer_name", UPDATE_JSON = UPDATE_JSON));
    }
    if info.size == 0 || info.size > INSTALLER_MAX {
        return Err(tf!("update.implausible_installer_size", UPDATE_JSON = UPDATE_JSON));
    }
    Ok(info)
}

/// A downloaded installer whose signatures, hash and version all checked out just now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pending {
    pub info: UpdateInfo,
    pub installer: PathBuf,
}

/// A [`Pending`] plus the installer opened so that nothing can change, replace or delete it
/// until this is dropped. The helper runs the installer while holding it.
struct Locked {
    pending: Pending,
    _file: fs::File,
}

/// What the update helper did: read once by the next start, then deleted.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateResult {
    pub version: String,
    pub ok: bool,
    /// The installer's exit code; `None` when it could not be started.
    pub code: Option<i32>,
    /// Unix seconds.
    pub at: u64,
}

/// Open `path` for reading while denying everyone else write and delete access (Windows), so
/// what was verified is what runs.
fn open_locked(path: &Path) -> io::Result<fs::File> {
    let mut o = fs::OpenOptions::new();
    o.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_SHARE_READ: u32 = 0x1;
        o.share_mode(FILE_SHARE_READ);
    }
    o.open(path)
}

/// Read at most `limit` bytes of `path`; more is an error.
fn read_small(path: &Path, limit: u64) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    fs::File::open(path)?.take(limit + 1).read_to_end(&mut out)?;
    if out.len() as u64 > limit {
        return Err(io::Error::other(format!("{}: too large", path.display())));
    }
    Ok(out)
}

/// Where downloads live: `<data dir>\updates\`.
pub struct Updates {
    dir: PathBuf,
}

impl Updates {
    pub fn new(data_dir: &Path) -> Updates {
        Updates { dir: data_dir.join("updates") }
    }

    fn data_dir(&self) -> Option<&Path> {
        self.dir.parent()
    }

    /// The pending update, when it fully verifies (see the module docs). Anything else in
    /// `updates\` is cleaned up.
    pub fn pending(&self) -> Option<Pending> {
        self.pending_with(CURRENT, &verify)
    }

    fn pending_with(&self, current: &str, verify: Verifier) -> Option<Pending> {
        self.locked_with(current, verify).map(|l| l.pending)
    }

    fn locked_with(&self, current: &str, verify: Verifier) -> Option<Locked> {
        let found = fs::read_dir(&self.dir)
            .ok()?
            .flatten()
            .filter(|e| e.path().is_dir())
            .find_map(|e| self.verify_dir(&e.path(), current, verify).ok());
        if found.is_none() {
            self.clear();
        }
        found
    }

    /// Check one `updates\<version>\` folder and lock its installer.
    fn verify_dir(&self, dir: &Path, current: &str, verify: Verifier) -> Result<Locked, String> {
        let json = read_small(&dir.join(UPDATE_JSON), UPDATE_JSON_MAX).map_err(|e| e.to_string())?;
        let sig = read_small(&dir.join(UPDATE_SIG), SIG_MAX).map_err(|e| e.to_string())?;
        let info = parse_info_with(&json, &String::from_utf8_lossy(&sig), verify)?;
        let version = &info.version;
        if dir.parent() != Some(self.dir.as_path())
            || dir.file_name().and_then(|n| n.to_str()) != Some(info.version.as_str())
        {
            return Err(tf!("update.pending_folder_mismatch", version = version));
        }
        if !is_newer(&info.version, current) {
            return Err(tf!("update.not_newer", version = version));
        }
        let installer = dir.join(&info.name);
        let mut file = open_locked(&installer).map_err(|e| e.to_string())?;
        let mut data = Vec::new();
        (&mut file).take(info.size + 1).read_to_end(&mut data).map_err(|e| e.to_string())?;
        check_installer(&data, &info, verify)?;
        Ok(Locked { pending: Pending { info, installer }, _file: file })
    }

    pub fn clear(&self) {
        let _ = fs::remove_dir_all(&self.dir);
    }

    fn failed_path(&self) -> Option<PathBuf> {
        self.data_dir().map(|d| d.join(FAILED_JSON))
    }

    /// The version whose installer failed, if one is remembered.
    pub fn failed_version(&self) -> Option<String> {
        let bytes = fs::read(self.failed_path()?).ok()?;
        serde_json::from_slice::<String>(&bytes).ok().filter(|v| parse_version(v).is_some())
    }

    /// Remember (or, with `None`, forget) the version whose installer failed.
    pub fn set_failed_version(&self, version: Option<&str>) {
        let Some(path) = self.failed_path() else { return };
        match version {
            Some(v) => {
                if let Ok(json) = serde_json::to_vec(v) {
                    let _ = fs::write(path, json);
                }
            }
            None => {
                let _ = fs::remove_file(path);
            }
        }
    }

    fn result_path(&self) -> Option<PathBuf> {
        self.data_dir().map(|d| d.join(RESULT_JSON))
    }

    /// The result the last update helper left behind, if any. Removed once read.
    pub fn take_result(&self) -> Option<UpdateResult> {
        let path = self.result_path()?;
        let bytes = fs::read(&path).ok()?;
        let _ = fs::remove_file(&path);
        serde_json::from_slice(&bytes).ok()
    }

    fn write_result(&self, res: &UpdateResult) {
        if let (Some(path), Ok(json)) = (self.result_path(), serde_json::to_vec_pretty(res)) {
            let _ = fs::write(path, json);
        }
    }

    /// Download and verify `release`'s installer, then keep it as the pending update.
    pub fn fetch(&self, remote: &HttpRemote, release: &Release) -> Result<Pending, String> {
        self.fetch_with(release, CURRENT, &verify, |dest| {
            remote.download_to(LATEST_TAG, &release.info.name, release.info.size, dest)
        })
    }

    fn fetch_with(
        &self,
        release: &Release,
        current: &str,
        verify: Verifier,
        download: impl FnOnce(&Path) -> Result<(), String>,
    ) -> Result<Pending, String> {
        if let Some(p) = self.pending_with(current, verify).filter(|p| p.info == release.info) {
            return Ok(p);
        }
        self.clear();
        let info = &release.info;
        let dir = self.dir.join(&info.version);
        let res = fs::create_dir_all(&dir)
            .map_err(|e| e.to_string())
            .and_then(|()| download(&dir.join(&info.name)))
            .and_then(|()| fs::write(dir.join(UPDATE_JSON), &release.json).map_err(|e| e.to_string()))
            .and_then(|()| fs::write(dir.join(UPDATE_SIG), &release.sig).map_err(|e| e.to_string()))
            .and_then(|()| self.verify_dir(&dir, current, verify));
        match res {
            Ok(locked) => Ok(locked.pending),
            Err(e) => {
                self.clear();
                Err(e)
            }
        }
    }
}

fn check_installer(data: &[u8], info: &UpdateInfo, verify: Verifier) -> Result<(), String> {
    if data.len() as u64 != info.size {
        return Err(t!("update.downloaded_installer_is_incomplete").into());
    }
    if sha256_bytes(data) != info.sha256.to_ascii_lowercase() {
        return Err(t!("update.installer_hash_does_not").into());
    }
    verify(data, &info.signature).map_err(|e| format!("installer: {e}"))
}

/// Ask the release for a newer version. `Ok(None)` = this build is the latest.
pub fn check(remote: &HttpRemote) -> Result<Option<Release>, String> {
    let json = remote.get_bytes(LATEST_TAG, UPDATE_JSON, UPDATE_JSON_MAX).map_err(|e| match e {
        Some(e) => e,
        None => tf!("update.no_update_json", UPDATE_JSON = UPDATE_JSON),
    })?;
    let sig = remote.get_bytes(LATEST_TAG, UPDATE_SIG, SIG_MAX).map_err(|e| match e {
        Some(e) => e,
        None => tf!("update.no_update_sig", sig_name = UPDATE_SIG),
    })?;
    let sig = String::from_utf8_lossy(&sig).into_owned();
    let info = parse_info(&json, &sig)?;
    Ok(is_newer(&info.version, CURRENT).then_some(Release { info, json, sig }))
}

// ---------------------------------------------------------------- applying

/// How the helper starts Patch Guard again after installing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relaunch {
    /// Do not start it (the user quit).
    No,
    /// Start it with its window.
    Window,
    /// Start it in the tray (it was started by autostart).
    Minimized,
}

/// Start the helper that installs the pending update in `data_dir` once this process has
/// exited. The caller quits right after.
pub fn apply(data_dir: &Path, relaunch: Relaunch) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let pid = std::process::id();
    // A fresh folder of our own: an exe started from a shared folder like %TEMP% would load
    // any DLL someone dropped next to it.
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = data_dir.join(HELPER_DIR).join(format!("{pid}-{nanos}"));
    fs::create_dir_all(&dir).map_err(|e| format!("helper update: {e}"))?;
    let helper = dir.join(HELPER_EXE);
    fs::copy(&exe, &helper).map_err(|e| format!("helper update: {e}"))?;
    let mut cmd = Command::new(&helper);
    cmd.current_dir(&dir).arg(ARG_APPLY).arg(ARG_WAIT_PID).arg(pid.to_string());
    match relaunch {
        Relaunch::No => {}
        Relaunch::Window => {
            cmd.arg(ARG_RELAUNCH);
        }
        Relaunch::Minimized => {
            cmd.arg(ARG_RELAUNCH).arg(ARG_MINIMIZED);
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }
    cmd.spawn().map(|_| ()).map_err(|e| format!("helper update: {e}"))
}

#[derive(Debug, PartialEq, Eq)]
struct HelperArgs {
    wait_pid: u32,
    relaunch: Relaunch,
}

/// Exactly `--apply-update --wait-pid <pid> [--relaunch [--minimized]]`, in any order.
/// Anything else (a path, an unknown flag, a repeated flag) is refused.
fn parse_helper_args(args: &[String]) -> Option<HelperArgs> {
    let mut it = args.iter();
    let (mut apply, mut pid, mut relaunch, mut minimized) = (false, None, false, false);
    while let Some(a) = it.next() {
        match a.as_str() {
            ARG_APPLY if !apply => apply = true,
            ARG_WAIT_PID if pid.is_none() => pid = Some(it.next()?.parse::<u32>().ok()?),
            ARG_RELAUNCH if !relaunch => relaunch = true,
            ARG_MINIMIZED if !minimized => minimized = true,
            _ => return None,
        }
    }
    let relaunch = match (relaunch, minimized) {
        (false, false) => Relaunch::No,
        (true, false) => Relaunch::Window,
        (true, true) => Relaunch::Minimized,
        (false, true) => return None,
    };
    (apply && pid.is_some_and(|p| p != 0)).then(|| HelperArgs { wait_pid: pid.unwrap_or(0), relaunch })
}

/// `main` calls this first: when the process was started as the update helper it does the
/// work and returns the exit code; otherwise `None` and the app starts normally.
///
/// `data_dir`: the app's data folder. `installed_exe`: where the installer put Patch Guard
/// (read from its uninstall entry), the only thing ever relaunched.
pub fn helper_main(
    data_dir: Option<PathBuf>,
    installed_exe: impl FnOnce() -> Option<PathBuf>,
) -> Option<i32> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.iter().any(|a| a == ARG_APPLY) {
        return None;
    }
    let (Some(h), Some(data_dir)) = (parse_helper_args(&args), data_dir) else { return Some(2) };
    wait_for_exit(h.wait_pid, Duration::from_secs(60));
    let updates = Updates::new(&data_dir);
    // Verified again here, and held open (no writes, no delete) until the installer is done.
    let Some(locked) = updates.locked_with(CURRENT, &verify) else { return Some(2) };
    let status = Command::new(&locked.pending.installer).arg("/S").status();
    let ok = status.as_ref().is_ok_and(|s| s.success());
    // Leave a note for the next start, so a failed install shows up in the History instead
    // of silently retrying every six hours.
    updates.write_result(&UpdateResult {
        version: locked.pending.info.version.clone(),
        ok,
        code: status.ok().and_then(|s| s.code()),
        at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    });
    // Installed or not, the pending update is done: a failing installer must not run again
    // on every start.
    drop(locked);
    updates.clear();
    if let Some(exe) = (h.relaunch != Relaunch::No).then(installed_exe).flatten() {
        let mut cmd = Command::new(exe);
        if h.relaunch == Relaunch::Minimized {
            cmd.arg(ARG_MINIMIZED);
        }
        let _ = cmd.spawn();
    }
    // The helper cannot delete its own folder while running; the next normal start does.
    Some(if ok { 0 } else { 1 })
}

/// Delete the helper copies earlier updates left behind (they could not delete themselves).
/// Called on a normal start; a copy that is somehow still running stays until next time.
pub fn clean_helpers(data_dir: &Path) {
    if let Ok(rd) = fs::read_dir(data_dir.join(HELPER_DIR)) {
        for d in rd.flatten() {
            let _ = fs::remove_dir_all(d.path());
        }
    }
    // v0.6.0 and earlier: `%TEMP%\wwm-patch-guard-updater-<pid>.exe`.
    let Ok(rd) = fs::read_dir(std::env::temp_dir()) else { return };
    for f in rd.flatten() {
        let name = f.file_name().to_string_lossy().into_owned();
        if name.starts_with(LEGACY_HELPER_PREFIX) && name.ends_with(".exe") {
            let _ = fs::remove_file(f.path());
        }
    }
}

fn wait_for_exit(pid: u32, timeout: Duration) {
    #[cfg(windows)]
    unsafe {
        use windows::Win32::Foundation::CloseHandle;
        use windows::Win32::System::Threading::{OpenProcess, PROCESS_SYNCHRONIZE, WaitForSingleObject};
        if let Ok(h) = OpenProcess(PROCESS_SYNCHRONIZE, false, pid) {
            let _ = WaitForSingleObject(h, timeout.as_millis() as u32);
            let _ = CloseHandle(h);
        }
    }
    #[cfg(not(windows))]
    {
        let _ = pid;
        std::thread::sleep(timeout.min(Duration::from_secs(2)));
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::io::Cursor;

    /// A throwaway key pair: (public key, sign fn) in cargo-packager's base64 format.
    pub(crate) fn test_key() -> (String, impl Fn(&[u8]) -> String) {
        let kp = minisign::KeyPair::generate_unencrypted_keypair().unwrap();
        let pk = STANDARD.encode(kp.pk.to_box().unwrap().to_string());
        let sk = kp.sk;
        let sign = move |data: &[u8]| {
            let b = minisign::sign(None, &sk, Cursor::new(data), Some("t"), None).unwrap();
            STANDARD.encode(b.to_string())
        };
        (pk, sign)
    }

    #[test]
    fn built_in_key_decodes() {
        let text = String::from_utf8(STANDARD.decode(PUBKEY.trim()).unwrap()).unwrap();
        minisign_verify::PublicKey::decode(&text).unwrap();
    }

    #[test]
    fn signature_checks() {
        let (pk, sign) = test_key();
        let sig = sign(b"hello");
        verify_with(&pk, b"hello", &sig).unwrap();
        assert!(verify_with(&pk, b"hellO", &sig).is_err());
        let (other, _) = test_key();
        assert!(verify_with(&other, b"hello", &sig).is_err());
        assert!(verify_with(&pk, b"hello", "not base64!").is_err());
        assert!(verify_with(&pk, b"hello", "").is_err());
    }

    #[test]
    fn versions() {
        assert_eq!(parse_version("v0.5.1"), Some((0, 5, 1)));
        assert_eq!(parse_version("1.2.3-beta"), Some((1, 2, 3)));
        assert_eq!(parse_version("1.2"), None);
        assert_eq!(parse_version("1.2.3.4"), None);
        assert!(is_newer("0.10.0", "0.9.9"));
        assert!(!is_newer("0.4.0", "0.4.0"));
        assert!(!is_newer("0.3.9", "0.4.0"));
        assert!(!is_newer("garbage", "0.4.0"));
    }

    fn args(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn helper_takes_no_paths() {
        let h = parse_helper_args(&args(&["--apply-update", "--wait-pid", "42", "--relaunch", "--minimized"]));
        assert_eq!(h, Some(HelperArgs { wait_pid: 42, relaunch: Relaunch::Minimized }));
        let h = parse_helper_args(&args(&["--wait-pid", "7", "--apply-update"]));
        assert_eq!(h, Some(HelperArgs { wait_pid: 7, relaunch: Relaunch::No }));
        // The v0.6.0 command line (installer path, --pending, --relaunch <exe>) is refused.
        for bad in [
            &["--apply-update", r"C:\x\evil.exe", "--wait-pid", "1"][..],
            &["--apply-update", "--wait-pid", "1", "--pending", r"C:\x\updates"],
            &["--apply-update", "--wait-pid", "1", "--relaunch", r"C:\x\evil.exe"],
            &["--apply-update", "--wait-pid", "1", "--version", "9.9.9"],
            &["--apply-update", "--wait-pid", "1", "--minimized"],
            &["--apply-update", "--apply-update", "--wait-pid", "1"],
            &["--apply-update", "--wait-pid", "0"],
            &["--apply-update"],
            &["--minimized"],
        ] {
            assert_eq!(parse_helper_args(&args(bad)), None, "{bad:?}");
        }
    }

    /// A signed release for version `v` with installer bytes `exe`, using `sign`.
    fn release(v: &str, exe: &[u8], sign: &dyn Fn(&[u8]) -> String) -> Release {
        let info = UpdateInfo {
            version: v.into(),
            name: "setup.exe".into(),
            sha256: sha256_bytes(exe),
            size: exe.len() as u64,
            signature: sign(exe),
            notes: String::new(),
            pub_date: String::new(),
        };
        let json = serde_json::to_vec(&info).unwrap();
        let sig = sign(&json);
        Release { info, json, sig }
    }

    type SignFn = Box<dyn Fn(&[u8]) -> String>;
    type VerifyFn = Box<dyn Fn(&[u8], &str) -> Result<(), String>>;

    struct Env {
        _tmp: tempfile::TempDir,
        u: Updates,
        sign: SignFn,
        verifier: VerifyFn,
    }

    fn env() -> Env {
        let tmp = tempfile::tempdir().unwrap();
        let u = Updates::new(tmp.path());
        let (pk, sign) = test_key();
        Env { _tmp: tmp, u, sign: Box::new(sign), verifier: Box::new(move |d, s| verify_with(&pk, d, s)) }
    }

    fn fetch(e: &Env, r: &Release, served: &[u8]) -> Result<Pending, String> {
        let served = served.to_vec();
        e.u.fetch_with(r, "1.0.0", &*e.verifier, |dest| fs::write(dest, &served).map_err(|e| e.to_string()))
    }

    #[test]
    fn a_verified_download_is_pending() {
        let e = env();
        let r = release("1.1.0", b"installer", &*e.sign);
        let p = fetch(&e, &r, b"installer").unwrap();
        assert_eq!(p.installer, e.u.dir.join("1.1.0").join("setup.exe"));
        assert_eq!(e.u.pending_with("1.0.0", &*e.verifier), Some(p));
        // Once this build is that version (or newer) it is not pending any more.
        assert_eq!(e.u.pending_with("1.1.0", &*e.verifier), None);
        assert!(!e.u.dir.exists(), "a stale pending update is cleaned up");
    }

    #[test]
    fn a_bad_download_is_never_pending() {
        let e = env();
        let r = release("1.1.0", b"installer", &*e.sign);
        assert!(fetch(&e, &r, b"installer!").is_err(), "wrong size");
        assert!(fetch(&e, &r, b"instal1er").is_err(), "wrong hash");
        assert!(!e.u.dir.exists());
        // Correct hash in a JSON signed by another key.
        let (_, other) = test_key();
        assert!(fetch(&e, &release("1.1.0", b"installer", &other), b"installer").is_err());
        // Not newer than the running build.
        assert!(fetch(&e, &release("1.0.0", b"installer", &*e.sign), b"installer").is_err());
    }

    #[test]
    fn tampering_after_the_download_is_caught_before_install() {
        let e = env();
        let r = release("1.1.0", b"installer", &*e.sign);
        let p = fetch(&e, &r, b"installer").unwrap();
        let ok = |e: &Env| e.u.pending_with("1.0.0", &*e.verifier).is_some();

        // Swapped installer with a matching (unsigned) hash written next to it: rejected,
        // because the hash and signature come from the signed JSON.
        fs::write(&p.installer, b"evilevil!").unwrap();
        assert!(!ok(&e));

        // A JSON of our own describing evil.exe: its signature does not match.
        let p = fetch(&e, &r, b"installer").unwrap();
        let dir = p.installer.parent().unwrap().to_path_buf();
        let mut info = r.info.clone();
        info.sha256 = sha256_bytes(b"evil");
        info.size = 4;
        fs::write(dir.join(UPDATE_JSON), serde_json::to_vec(&info).unwrap()).unwrap();
        fs::write(&p.installer, b"evil").unwrap();
        assert!(!ok(&e));

        // A signed update moved into a folder named after another version.
        let p = fetch(&e, &r, b"installer").unwrap();
        let dir = p.installer.parent().unwrap().to_path_buf();
        fs::rename(&dir, e.u.dir.join("9.9.9")).unwrap();
        assert!(!ok(&e));
        assert!(!e.u.dir.exists());
    }

    #[test]
    fn a_locked_installer_cannot_be_replaced() {
        let e = env();
        let r = release("1.1.0", b"installer", &*e.sign);
        let p = fetch(&e, &r, b"installer").unwrap();
        let locked = e.u.locked_with("1.0.0", &*e.verifier).unwrap();
        #[cfg(windows)]
        {
            assert!(fs::write(&p.installer, b"evil").is_err(), "no writes while locked");
            assert!(fs::remove_file(&p.installer).is_err(), "no delete while locked");
            assert!(fs::rename(&p.installer, p.installer.with_extension("old")).is_err());
        }
        drop(locked);
        assert_eq!(fs::read(&p.installer).unwrap(), b"installer");
    }

    /// The installer runs while the helper holds it locked (Windows lets a process start from
    /// a file opened with read sharing only).
    #[cfg(windows)]
    #[test]
    fn a_locked_installer_still_runs() {
        let tmp = tempfile::tempdir().unwrap();
        let exe = tmp.path().join("whoami.exe");
        let sys = std::env::var_os("SystemRoot").map(PathBuf::from).unwrap_or(r"C:\Windows".into());
        fs::copy(sys.join(r"System32\whoami.exe"), &exe).unwrap();
        let lock = open_locked(&exe).unwrap();
        let out = Command::new(&exe).output().unwrap();
        assert!(out.status.success());
        drop(lock);
    }

    #[test]
    fn update_result_is_read_once() {
        let tmp = tempfile::tempdir().unwrap();
        let u = Updates::new(tmp.path());
        assert_eq!(u.take_result(), None);
        let res = UpdateResult { version: "0.6.0".into(), ok: false, code: Some(2), at: 7 };
        u.write_result(&res);
        assert!(tmp.path().join(RESULT_JSON).is_file());
        assert_eq!(u.take_result(), Some(res));
        assert_eq!(u.take_result(), None, "deleted after the first read");
    }

    #[test]
    fn failed_version_survives_until_cleared() {
        let tmp = tempfile::tempdir().unwrap();
        let u = Updates::new(tmp.path());
        assert_eq!(u.failed_version(), None);
        u.set_failed_version(Some("0.6.1"));
        // A new `Updates` = the next start of the app.
        assert_eq!(Updates::new(tmp.path()).failed_version().as_deref(), Some("0.6.1"));
        u.clear(); // a cleared download folder does not forget it
        assert_eq!(u.failed_version().as_deref(), Some("0.6.1"));
        u.set_failed_version(None);
        assert_eq!(u.failed_version(), None);
    }

    #[test]
    fn helper_copies_are_cleaned_up() {
        let tmp = tempfile::tempdir().unwrap();
        let old = tmp.path().join(HELPER_DIR).join("1-2");
        fs::create_dir_all(&old).unwrap();
        fs::write(old.join(HELPER_EXE), b"x").unwrap();
        clean_helpers(tmp.path());
        assert!(!old.exists());
    }

    #[test]
    fn info_rejects_paths_and_silly_sizes() {
        let j = |name: &str, size: u64, version: &str| {
            serde_json::json!({"version": version, "name": name, "sha256": "x", "size": size,
                               "signature": "s"})
            .to_string()
        };
        assert!(decode_info(j("setup.exe", 10, "1.0.0").as_bytes()).is_ok());
        assert!(decode_info(j(r"..\evil.exe", 10, "1.0.0").as_bytes()).is_err());
        assert!(decode_info(j("a/b.exe", 10, "1.0.0").as_bytes()).is_err());
        assert!(decode_info(j("setup.bat", 10, "1.0.0").as_bytes()).is_err());
        assert!(decode_info(j("setup.exe", 0, "1.0.0").as_bytes()).is_err());
        assert!(decode_info(j("setup.exe", 1 << 40, "1.0.0").as_bytes()).is_err());
        assert!(decode_info(j("setup.exe", 10, "latest").as_bytes()).is_err());
        // The version names a folder: `1.0.0-<anything>` parses as a version but is no name.
        assert!(decode_info(j("setup.exe", 10, r"1.0.0-..\..\x").as_bytes()).is_err());
        // Without a valid signature nothing is even parsed.
        assert!(parse_info(j("setup.exe", 10, "1.0.0").as_bytes(), "").is_err());
    }
}
