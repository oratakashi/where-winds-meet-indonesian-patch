//! Small Windows queries: OS build, Mica support, accent colour, data folder.
use std::path::PathBuf;
use std::sync::OnceLock;

/// The identifier v0.3 (Tauri) used: data folder name and notification AppUserModelID.
pub const IDENTIFIER: &str = "com.oratakashi.wwm";
pub const PRODUCT_NAME: &str = "WWM Patch Guard";

/// Windows build number (`22000`+ = Windows 11), 0 when unknown or not on Windows.
pub fn windows_build() -> u32 {
    static BUILD: OnceLock<u32> = OnceLock::new();
    *BUILD.get_or_init(|| {
        #[cfg(windows)]
        unsafe {
            let mut v: windows::Win32::System::SystemInformation::OSVERSIONINFOW =
                std::mem::zeroed();
            v.dwOSVersionInfoSize = std::mem::size_of_val(&v) as u32;
            if windows::Wdk::System::SystemServices::RtlGetVersion(&mut v).is_ok() {
                return v.dwBuildNumber;
            }
        }
        0
    })
}

/// GPUI only applies the Mica backdrop from Windows 11 22H2 (build 22621); the page must stay
/// opaque everywhere else.
pub fn mica_supported() -> bool {
    windows_build() >= 22621
}

/// The 7 shades of the user's accent colour (Light3 … Dark3) as `0xRRGGBB`.
pub fn accent_palette() -> Option<[u32; 7]> {
    #[cfg(windows)]
    {
        use winreg::RegKey;
        use winreg::enums::HKEY_CURRENT_USER;
        let key = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Accent")
            .ok()?;
        let raw = key.get_raw_value("AccentPalette").ok()?;
        // 8 entries of R, G, B, A.
        if raw.bytes.len() < 28 {
            return None;
        }
        let mut out = [0u32; 7];
        for (i, o) in out.iter_mut().enumerate() {
            let b = &raw.bytes[i * 4..i * 4 + 3];
            *o = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
        }
        Some(out)
    }
    #[cfg(not(windows))]
    None
}

/// The exe name the installer puts in its folder (`binaries` in `[package.metadata.packager]`).
pub const MAIN_EXE: &str = "wwm-patch-guard.exe";

/// `%LOCALAPPDATA%\com.oratakashi.wwm` — the same folder Tauri's `app_local_data_dir` gave v0.3,
/// so its config and history carry over. Debug builds accept `WWM_PATCH_GUARD_DATA_DIR`
/// instead (testing); release builds ignore it, so the environment cannot move the data — and
/// the update the helper installs — somewhere else.
pub fn data_dir() -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    if let Some(d) = std::env::var_os("WWM_PATCH_GUARD_DATA_DIR").filter(|d| !d.is_empty()) {
        return Some(PathBuf::from(d));
    }
    dirs::data_local_dir().map(|d| d.join(IDENTIFIER))
}

/// Where the per-user NSIS installer put Patch Guard: its uninstall entry's `InstallLocation`
/// (`HKCU\…\Uninstall\WWM Patch Guard`, written by cargo-packager's template) plus
/// [`MAIN_EXE`]. `None` when it is not installed (a dev build): the update helper then starts
/// nothing.
pub fn installed_exe() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        use winreg::RegKey;
        use winreg::enums::HKEY_CURRENT_USER;
        let key = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey(format!(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{PRODUCT_NAME}"))
            .ok()?;
        let loc: String = key.get_value("InstallLocation").ok()?;
        let exe = PathBuf::from(loc.trim().trim_matches('"')).join(MAIN_EXE);
        exe.is_file().then_some(exe)
    }
    #[cfg(not(windows))]
    None
}

/// Load DLLs only from the exe's folder and System32, never from the current folder or `PATH`.
/// Called first thing in `main`.
pub fn harden_dll_search() {
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::LibraryLoader::{
            LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, SetDefaultDllDirectories,
        };
        let _ = SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
    }
}
