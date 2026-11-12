//! WWM Patch Guard — keeps the Indonesian patch installed after NetEase's patcher puts the
//! original locale files back. See `guard.rs` for the rules; this file is the Tauri shell
//! (tray, window, commands, notifications).
pub mod engine;
pub mod guard;
pub mod locations;
pub mod model;
pub mod remote;
pub mod store;

use engine::{Engine, Msg, Snapshot};
use model::Event;
use serde::Serialize;
use std::path::PathBuf;
use store::Store;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_notification::NotificationExt;

/// Passed by the autostart entry: start in the tray without opening the window.
const ARG_MINIMIZED: &str = "--minimized";

#[derive(Serialize)]
struct Ui {
    #[serde(flatten)]
    snapshot: Snapshot,
    autostart: bool,
    version: &'static str,
}

#[tauri::command]
fn get_status(app: AppHandle, engine: State<Engine>) -> Ui {
    Ui {
        snapshot: engine.snapshot(),
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
        version: env!("CARGO_PKG_VERSION"),
    }
}

/// Ask for a folder and guard every locale folder found from it.
/// `Ok(None)`: cancelled. `Ok(Some(n))`: n folders added.
#[tauri::command]
async fn add_folder(app: AppHandle, engine: State<'_, Engine>) -> Result<Option<usize>, String> {
    let Some(picked) = app
        .dialog()
        .file()
        .set_title("Pilih folder instalasi Where Winds Meet")
        .blocking_pick_folder()
    else {
        return Ok(None);
    };
    let picked = picked.into_path().map_err(|e| e.to_string())?;
    let found = locations::detect(&picked);
    if found.is_empty() {
        return Err(
            "Tidak ada file bahasa game (translate_words_map_*) di folder itu. \
             Pilih folder instalasi game atau launcher-nya."
                .into(),
        );
    }
    let mut added = 0;
    engine.update_config(|c| {
        for g in found {
            if !c.dirs.contains(&g) {
                c.dirs.push(g);
                added += 1;
            }
        }
    })?;
    Ok(Some(added))
}

#[tauri::command]
fn remove_folder(engine: State<Engine>, path: PathBuf) -> Result<(), String> {
    engine.update_config(|c| c.dirs.retain(|d| d.path != path))
}

#[tauri::command]
fn check_now(engine: State<Engine>) {
    engine.send(Msg::Check { force_remote: true });
}

#[tauri::command]
fn set_paused(engine: State<Engine>, paused: bool) -> Result<(), String> {
    engine.update_config(|c| c.paused = paused)
}

#[tauri::command]
fn set_auto_update(engine: State<Engine>, enabled: bool) -> Result<(), String> {
    engine.update_config(|c| c.auto_update = enabled)
}

#[tauri::command]
fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    let auto = app.autolaunch();
    if enabled { auto.enable() } else { auto.disable() }.map_err(|e| e.to_string())
}

/// Put the original files back and pause the guard, so it does not patch them again.
#[tauri::command]
fn restore_originals(engine: State<Engine>) -> Result<(), String> {
    engine.update_config(|c| c.paused = true)?;
    engine.send(Msg::Uninstall);
    Ok(())
}

/// Install the latest release into a folder even where the installed file is unknown.
#[tauri::command]
fn force_install(engine: State<Engine>, path: PathBuf) -> Result<(), String> {
    if !engine.config().dirs.iter().any(|d| d.path == path) {
        return Err("folder itu tidak dijaga".into());
    }
    engine.send(Msg::ForceInstall(path));
    Ok(())
}

fn show_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
        return;
    }
    let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("WWM Patch Guard")
        .inner_size(560.0, 720.0)
        .min_inner_size(440.0, 520.0)
        .build();
}

fn notify(app: &AppHandle, events: &[Event]) {
    let restored = events.iter().filter(|e| matches!(e, Event::Restored { .. })).count();
    let installed = events.iter().filter(|e| matches!(e, Event::Installed { .. })).count();
    let title = match (restored, installed) {
        (0, 0) => return,
        (_, 0) => "Patch Indonesia dipasang kembali",
        _ => "Patch Indonesia diperbarui",
    };
    let _ = app
        .notification()
        .builder()
        .title(title)
        .body(format!(
            "{} file dipulihkan. Kalau game sedang berjalan, restart game supaya teksnya berubah.",
            restored + installed
        ))
        .show();
}

fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Buka Patch Guard", true, None::<&str>)?;
    let check = MenuItem::with_id(app, "check", "Periksa sekarang", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Keluar", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &check, &quit])?;
    let mut tray = TrayIconBuilder::with_id("main")
        .tooltip("WWM Patch Guard")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, e| match e.id.as_ref() {
            "open" => show_window(app),
            "check" => app.state::<Engine>().send(Msg::Check { force_remote: true }),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, e| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = e
            {
                show_window(tray.app_handle());
            }
        });
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| show_window(app)))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![ARG_MINIMIZED]),
        ))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let handle = app.handle().clone();
            // WWM_PATCH_GUARD_DATA_DIR: keep config/cache somewhere else (testing).
            let data_dir = match std::env::var_os("WWM_PATCH_GUARD_DATA_DIR") {
                Some(d) if !d.is_empty() => PathBuf::from(d),
                _ => app.path().app_local_data_dir()?,
            };
            let store = Store::new(data_dir)?;
            let engine = Engine::start(store, move |events| {
                notify(&handle, events);
                let _ = handle.emit("status", ());
            });
            app.manage(engine);
            build_tray(app.handle())?;
            if !std::env::args().any(|a| a == ARG_MINIMIZED) {
                show_window(app.handle());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            add_folder,
            remove_folder,
            check_now,
            set_paused,
            set_auto_update,
            set_autostart,
            restore_originals,
            force_install
        ])
        .build(tauri::generate_context!())
        .expect("error while building WWM Patch Guard")
        // Closing the window destroys it (and its WebView2 processes); the guard keeps
        // running in the tray until "Keluar".
        .run(|_app, event| {
            if let RunEvent::ExitRequested { api, code, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
