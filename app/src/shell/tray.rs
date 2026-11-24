//! The tray icon. Its events arrive on the main thread's message loop (GPUI pumps it) and are
//! forwarded as [`Cmd`]s to the foreground task in `app.rs`.
use super::system::PRODUCT_NAME;
use crate::app::Cmd;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

/// The tray icon plus its menu items (relabelled when the language or a pending update changes).
pub struct Tray {
    _icon: TrayIcon,
    open: MenuItem,
    check: MenuItem,
    update: MenuItem,
    quit: MenuItem,
}

fn open_label() -> &'static str {
    t!("tray.open_patch_guard")
}

fn check_label() -> &'static str {
    t!("common.check_now")
}

fn quit_label() -> &'static str {
    t!("tray.quit")
}

fn no_update_label() -> &'static str {
    t!("tray.no_patch_guard_update")
}

impl Tray {
    /// `Some(version)`: a downloaded update is waiting; offer the restart.
    pub fn set_update(&self, version: Option<&str>) {
        match version {
            Some(v) => {
                self.update.set_text(tf!("tray.restart_to_update_to", v = v));
                self.update.set_enabled(true);
            }
            None => {
                self.update.set_text(no_update_label());
                self.update.set_enabled(false);
            }
        }
    }

    /// Every item again, in the current language.
    pub fn relabel(&self, update: Option<&str>) {
        self.open.set_text(open_label());
        self.check.set_text(check_label());
        self.quit.set_text(quit_label());
        self.set_update(update);
    }
}

/// Must be called on the main thread; keep the returned icon alive for as long as it should show.
pub fn build(tx: async_channel::Sender<Cmd>) -> Result<Tray, String> {
    let open = MenuItem::with_id("open", open_label(), true, None);
    let check = MenuItem::with_id("check", check_label(), true, None);
    let update = MenuItem::with_id("update", no_update_label(), false, None);
    let quit = MenuItem::with_id("quit", quit_label(), true, None);
    let menu = Menu::with_items(&[&open, &check, &update, &quit]).map_err(|e| e.to_string())?;

    let menu_tx = tx.clone();
    MenuEvent::set_event_handler(Some(move |e: MenuEvent| {
        let cmd = match e.id.as_ref() {
            "open" => Cmd::Show,
            "check" => Cmd::CheckNow,
            "update" => Cmd::RestartForUpdate,
            "quit" => Cmd::Quit,
            _ => return,
        };
        let _ = menu_tx.try_send(cmd);
    }));
    TrayIconEvent::set_event_handler(Some(move |e: TrayIconEvent| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } = e
        {
            let _ = tx.try_send(Cmd::Show);
        }
    }));

    let mut builder = TrayIconBuilder::new()
        .with_tooltip(PRODUCT_NAME)
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false);
    // Resource 1 is the app icon embedded by build.rs.
    #[cfg(windows)]
    if let Ok(icon) = Icon::from_resource(1, Some((32, 32))) {
        builder = builder.with_icon(icon);
    }
    #[cfg(not(windows))]
    let _ = Icon::from_rgba;
    let icon = builder.build().map_err(|e| e.to_string())?;
    Ok(Tray { _icon: icon, open, check, update, quit })
}
