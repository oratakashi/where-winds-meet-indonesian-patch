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
//! Applying never happens in this process: [`apply`] copies the running exe to `%TEMP%` and
//! starts that copy as a helper (`--apply-update`), which waits for us to exit, runs the
//! installer silently (`/S`, per-user install = no UAC) and starts Patch Guard again. The copy
//! is needed because the installer replaces the exe the app runs from.
use crate::remote::{HttpRemote, LATEST_TAG};
use crate::store::sha256_file;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// The update description on the `latest` release.
pub const UPDATE_JSON: &str = "patch-guard-update.json";
/// The minisign public key (cargo-packager format: base64 of the `.pub` text).
const PUBKEY: &str = include_str!("../../signing/patch-guard.pub");
/// Largest installer we accept, whatever the JSON says.
const INSTALLER_MAX: u64 = 200 << 20;

pub const ARG_APPLY: &str = "--apply-update";
const ARG_WAIT_PID: &str = "--wait-pid";
const ARG_RELAUNCH: &str = "--relaunch";
const ARG_PENDING: &str = "--pending";

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
        let raw = STANDARD.decode(b64.trim()).map_err(|_| "bukan base64".to_string())?;
        String::from_utf8(raw).map_err(|_| "bukan teks".to_string())
    };
    let pk = minisign_verify::PublicKey::decode(&text(pubkey_b64)?)
        .map_err(|e| format!("public key rusak: {e}"))?;
    let sig = minisign_verify::Signature::decode(&text(sig_b64)?)
        .map_err(|e| format!("tanda tangan rusak: {e}"))?;
    pk.verify(data, &sig, false).map_err(|_| "tanda tangan tidak cocok".to_string())
}

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

/// Verify and parse `patch-guard-update.json` against its `.sig`.
pub fn parse_info(json: &[u8], sig: &str) -> Result<UpdateInfo, String> {
    verify(json, sig).map_err(|e| format!("{UPDATE_JSON}: {e}"))?;
    decode_info(json)
}

fn decode_info(json: &[u8]) -> Result<UpdateInfo, String> {
    let info: UpdateInfo =
        serde_json::from_slice(json).map_err(|e| format!("{UPDATE_JSON} tidak terbaca: {e}"))?;
    if parse_version(&info.version).is_none() {
        return Err(format!("{UPDATE_JSON}: versi '{}' tidak valid", info.version));
    }
    // A name, not a path: the installer is always fetched from the same release.
    if info.name.is_empty() || info.name.contains(['/', '\\']) || info.name.contains("..") {
        return Err(format!("{UPDATE_JSON}: nama installer tidak valid"));
    }
    if info.size == 0 || info.size > INSTALLER_MAX {
        return Err(format!("{UPDATE_JSON}: ukuran installer tidak masuk akal"));
    }
    Ok(info)
}

/// A downloaded, verified installer waiting to be run (`updates/pending.json`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Pending {
    pub info: UpdateInfo,
    pub installer: PathBuf,
}

/// Where downloads live: `<data dir>\updates\`.
pub struct Updates {
    dir: PathBuf,
}

impl Updates {
    pub fn new(data_dir: &Path) -> Updates {
        Updates { dir: data_dir.join("updates") }
    }

    fn marker(&self) -> PathBuf {
        self.dir.join("pending.json")
    }

    /// The pending update, when it is still newer than this build and its installer still
    /// matches what was verified. Anything else is cleaned up.
    pub fn pending(&self) -> Option<Pending> {
        let p: Pending = serde_json::from_slice(&fs::read(self.marker()).ok()?).ok()?;
        let ok = is_newer(&p.info.version, CURRENT)
            && p.installer.starts_with(&self.dir)
            && sha256_file(&p.installer).is_ok_and(|h| h == p.info.sha256);
        if ok {
            Some(p)
        } else {
            self.clear();
            None
        }
    }

    pub fn clear(&self) {
        let _ = fs::remove_dir_all(&self.dir);
    }

    /// Download and verify `info`'s installer, then record it as pending.
    pub fn fetch(&self, remote: &HttpRemote, info: &UpdateInfo) -> Result<Pending, String> {
        if let Some(p) = self.pending().filter(|p| p.info == *info) {
            return Ok(p);
        }
        self.clear();
        let dir = self.dir.join(&info.version);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let installer = dir.join(&info.name);
        let res = remote
            .download_to(LATEST_TAG, &info.name, info.size, &installer)
            .and_then(|()| check_installer(&installer, info));
        if let Err(e) = res {
            self.clear();
            return Err(e);
        }
        let p = Pending { info: info.clone(), installer };
        let json = serde_json::to_vec_pretty(&p).map_err(|e| e.to_string())?;
        fs::write(self.marker(), json).map_err(|e| e.to_string())?;
        Ok(p)
    }
}

fn check_installer(path: &Path, info: &UpdateInfo) -> Result<(), String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    if data.len() as u64 != info.size {
        return Err("installer terunduh tidak lengkap".into());
    }
    let sha = sha256_file(path).map_err(|e| e.to_string())?;
    if sha != info.sha256 {
        return Err("hash installer tidak cocok".into());
    }
    verify(&data, &info.signature).map_err(|e| format!("installer: {e}"))
}

/// Ask the release for a newer version. `Ok(None)` = this build is the latest.
pub fn check(remote: &HttpRemote) -> Result<Option<UpdateInfo>, String> {
    let json = remote.get_bytes(LATEST_TAG, UPDATE_JSON, 64 << 10).map_err(|e| match e {
        Some(e) => e,
        None => format!("rilis belum punya {UPDATE_JSON}"),
    })?;
    let sig_name = format!("{UPDATE_JSON}.sig");
    let sig = remote.get_bytes(LATEST_TAG, &sig_name, 4096).map_err(|e| match e {
        Some(e) => e,
        None => format!("rilis belum punya {sig_name}"),
    })?;
    let info = parse_info(&json, &String::from_utf8_lossy(&sig))?;
    Ok(is_newer(&info.version, CURRENT).then_some(info))
}

// ---------------------------------------------------------------- applying

/// Start the helper that installs `p` once this process has exited. The caller quits right
/// after. `relaunch`: start Patch Guard again afterwards, with these arguments.
pub fn apply(p: &Pending, relaunch: Option<&[String]>) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let pid = std::process::id();
    let helper = std::env::temp_dir().join(format!("wwm-patch-guard-updater-{pid}.exe"));
    fs::copy(&exe, &helper).map_err(|e| format!("helper update: {e}"))?;
    let mut cmd = Command::new(&helper);
    cmd.arg(ARG_APPLY)
        .arg(&p.installer)
        .arg(ARG_WAIT_PID)
        .arg(pid.to_string())
        .arg(ARG_PENDING)
        .arg(p.installer.parent().and_then(Path::parent).unwrap_or(Path::new("")));
    if let Some(args) = relaunch {
        cmd.arg(ARG_RELAUNCH).arg(&exe).arg("--").args(args);
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
    installer: PathBuf,
    wait_pid: u32,
    updates_dir: PathBuf,
    relaunch: Option<(PathBuf, Vec<String>)>,
}

fn parse_helper_args(args: &[String]) -> Option<HelperArgs> {
    let mut it = args.iter();
    let mut out = HelperArgs {
        installer: PathBuf::new(),
        wait_pid: 0,
        updates_dir: PathBuf::new(),
        relaunch: None,
    };
    let mut seen_apply = false;
    while let Some(a) = it.next() {
        match a.as_str() {
            ARG_APPLY => {
                out.installer = it.next()?.into();
                seen_apply = true;
            }
            ARG_WAIT_PID => out.wait_pid = it.next()?.parse().ok()?,
            ARG_PENDING => out.updates_dir = it.next()?.into(),
            ARG_RELAUNCH => {
                let exe = PathBuf::from(it.next()?);
                let rest: Vec<String> = match it.next().map(String::as_str) {
                    Some("--") => it.by_ref().cloned().collect(),
                    Some(_) => return None,
                    None => Vec::new(),
                };
                out.relaunch = Some((exe, rest));
            }
            _ => {}
        }
    }
    seen_apply.then_some(out)
}

/// `main` calls this first: when the process was started as the update helper it does the
/// work and returns the exit code; otherwise `None` and the app starts normally.
pub fn helper_main() -> Option<i32> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.iter().any(|a| a == ARG_APPLY) {
        return None;
    }
    let Some(h) = parse_helper_args(&args) else { return Some(2) };
    wait_for_exit(h.wait_pid, Duration::from_secs(60));
    let ok = Command::new(&h.installer).arg("/S").status().is_ok_and(|s| s.success());
    // Installed or not, the pending update is done: a failing installer must not run again
    // on every start.
    if h.updates_dir.ends_with("updates") {
        let _ = fs::remove_dir_all(&h.updates_dir);
    }
    if let Some((exe, rest)) = h.relaunch {
        let _ = Command::new(exe).args(rest).spawn();
    }
    // The helper is a copy in %TEMP%; it cannot delete itself while running. Windows cleans
    // the temp folder, and the next helper uses another name.
    Some(if ok { 0 } else { 1 })
}

/// Delete the helper copies earlier updates left in `%TEMP%` (they could not delete
/// themselves). Called on a normal start, when none of them is running any more.
pub fn clean_helpers() {
    let Ok(rd) = fs::read_dir(std::env::temp_dir()) else { return };
    for f in rd.flatten() {
        let name = f.file_name().to_string_lossy().into_owned();
        if name.starts_with("wwm-patch-guard-updater-") && name.ends_with(".exe") {
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

    #[test]
    fn helper_args_round_trip() {
        let a: Vec<String> = [
            "--apply-update", r"C:\u\updates\0.5.1\setup.exe", "--wait-pid", "42", "--pending",
            r"C:\u\updates", "--relaunch", r"C:\app\wwm.exe", "--", "--minimized",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let h = parse_helper_args(&a).unwrap();
        assert_eq!(h.wait_pid, 42);
        assert_eq!(h.installer, PathBuf::from(r"C:\u\updates\0.5.1\setup.exe"));
        assert_eq!(h.relaunch, Some((PathBuf::from(r"C:\app\wwm.exe"), vec!["--minimized".into()])));
        assert!(parse_helper_args(&["--minimized".to_string()]).is_none());
    }

    #[test]
    fn pending_is_dropped_when_not_newer_or_changed() {
        let tmp = tempfile::tempdir().unwrap();
        let u = Updates::new(tmp.path());
        fs::create_dir_all(u.dir.join("9.0.0")).unwrap();
        let installer = u.dir.join("9.0.0").join("setup.exe");
        fs::write(&installer, b"installer").unwrap();
        let info = UpdateInfo {
            version: "9.0.0".into(),
            name: "setup.exe".into(),
            sha256: sha256_file(&installer).unwrap(),
            size: 9,
            signature: String::new(),
            notes: String::new(),
            pub_date: String::new(),
        };
        let write = |p: &Pending| fs::write(u.marker(), serde_json::to_vec(p).unwrap()).unwrap();
        let p = Pending { info: info.clone(), installer: installer.clone() };
        write(&p);
        assert_eq!(u.pending(), Some(p.clone()));

        fs::write(&installer, b"tampered!").unwrap();
        assert_eq!(u.pending(), None);
        assert!(!u.dir.exists(), "a bad pending update is cleaned up");

        fs::create_dir_all(u.dir.join("9.0.0")).unwrap();
        fs::write(&installer, b"installer").unwrap();
        write(&Pending { info: UpdateInfo { version: "0.0.1".into(), ..info }, installer });
        assert_eq!(u.pending(), None);
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
        assert!(decode_info(j("setup.exe", 0, "1.0.0").as_bytes()).is_err());
        assert!(decode_info(j("setup.exe", 1 << 40, "1.0.0").as_bytes()).is_err());
        assert!(decode_info(j("setup.exe", 10, "latest").as_bytes()).is_err());
        // Without a valid signature nothing is even parsed.
        assert!(parse_info(j("setup.exe", 10, "1.0.0").as_bytes(), "").is_err());
    }
}
