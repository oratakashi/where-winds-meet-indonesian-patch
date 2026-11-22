//! The tray icon. Its events arrive on the main thread's message loop (GPUI pumps it) and are
//! forwarded as [`Cmd`]s to the foreground task in `app.rs`.
use super::system::PRODUCT_NAME;
use crate::app::Cmd;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

/// Must be called on the main thread; keep the returned icon alive for as long as it should show.
pub fn build(tx: async_channel::Sender<Cmd>) -> Result<TrayIcon, String> {
    let open = MenuItem::with_id("open", "Buka Patch Guard", true, None);
    let check = MenuItem::with_id("check", "Periksa sekarang", true, None);
    let quit = MenuItem::with_id("quit", "Keluar", true, None);
    let menu = Menu::with_items(&[&open, &check, &quit]).map_err(|e| e.to_string())?;

    let menu_tx = tx.clone();
    MenuEvent::set_event_handler(Some(move |e: MenuEvent| {
        let cmd = match e.id.as_ref() {
            "open" => Cmd::Show,
            "check" => Cmd::CheckNow,
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
    builder.build().map_err(|e| e.to_string())
}
