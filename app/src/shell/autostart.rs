//! "Run when Windows starts": a `HKCU\…\CurrentVersion\Run` entry named like v0.3's
//! (`tauri-plugin-autostart` used the product name), so an existing entry is recognised and
//! never duplicated.
use super::system::PRODUCT_NAME;
use auto_launch::{AutoLaunch, AutoLaunchBuilder, WindowsEnableMode};

/// Passed by the autostart entry: start in the tray without opening the window.
pub const ARG_MINIMIZED: &str = "--minimized";

fn entry() -> Result<AutoLaunch, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    AutoLaunchBuilder::new()
        .set_app_name(PRODUCT_NAME)
        .set_app_path(&exe.display().to_string())
        .set_args(&[ARG_MINIMIZED])
        .set_windows_enable_mode(WindowsEnableMode::CurrentUser)
        .build()
        .map_err(|e| e.to_string())
}

pub fn is_enabled() -> bool {
    entry().and_then(|a| a.is_enabled().map_err(|e| e.to_string())).unwrap_or(false)
}

pub fn set(enabled: bool) -> Result<(), String> {
    let a = entry()?;
    if enabled { a.enable() } else { a.disable() }.map_err(|e| e.to_string())
}
