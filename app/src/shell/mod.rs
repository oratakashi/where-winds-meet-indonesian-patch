//! OS integration that Tauri plugins provided in v0.3: tray, single instance, autostart,
//! plus a few Windows queries (accent colour, Mica support, data folder).
pub mod autostart;
pub mod instance;
pub mod system;
pub mod tray;
