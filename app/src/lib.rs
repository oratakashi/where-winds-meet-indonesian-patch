//! WWM Patch Guard — keeps the Indonesian patch installed after NetEase's patcher puts the
//! original locale files back. `core/` holds the rules and the background engine (no UI);
//! `shell/` the OS integration (tray, single instance, autostart); `fluent/` + `pages/` the
//! GPUI window, `app.rs` ties it together.
pub mod app;
pub mod core;
pub mod fluent;
pub mod pages;
pub mod shell;
pub mod state;

pub use core::{engine, guard, locations, model, remote, store, update};
