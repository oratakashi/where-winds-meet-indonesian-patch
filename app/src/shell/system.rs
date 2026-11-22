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

/// `%LOCALAPPDATA%\com.oratakashi.wwm` — the same folder Tauri's `app_local_data_dir` gave v0.3,
/// so its config and history carry over. `WWM_PATCH_GUARD_DATA_DIR` overrides it (testing).
pub fn data_dir() -> Option<PathBuf> {
    match std::env::var_os("WWM_PATCH_GUARD_DATA_DIR") {
        Some(d) if !d.is_empty() => Some(PathBuf::from(d)),
        _ => dirs::data_local_dir().map(|d| d.join(IDENTIFIER)),
    }
}
