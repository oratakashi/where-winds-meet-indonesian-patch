//! The application: single instance, engine, tray, notifications, and the window — navigation
//! pane on the left (WinUI `NavigationView` style), the selected page on the right.
use crate::engine::Engine;
use crate::fluent::{Button, ButtonStyle, Theme, error_bar, glyph, icon, theme};
use crate::model::Event;
use crate::pages::{history, home, settings};
use crate::shell::system::{self, IDENTIFIER, PRODUCT_NAME};
use crate::shell::{autostart, instance, tray};
use crate::state::{AppState, Page};
use crate::store::Store;
use crate::update::{self, Relaunch, Updates};
use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, Entity, FocusHandle, FontWeight, KeyBinding, QuitMode, Role, SharedString,
    Subscription, SystemNotification, TitlebarOptions, Window, WindowBackgroundAppearance,
    WindowBounds, WindowOptions, actions, div, px, size,
};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

actions!(patch_guard, [FocusNext, FocusPrev, Dismiss]);

/// How long the first guard pass waits when autostart launched the app at login.
const FIRST_PASS_AT_LOGIN: Duration = Duration::from_secs(30);

/// Work for the main thread, sent from the tray, the second-instance listener and the engine.
pub enum Cmd {
    Show,
    CheckNow,
    /// Install the downloaded Patch Guard update and start again.
    RestartForUpdate,
    /// Exit; a downloaded update is installed on the way out (no restart).
    Quit,
    /// The engine finished (or started) a pass; non-empty when it changed files.
    Engine(Vec<Event>),
    /// The UI language changed: relabel the tray menu.
    LanguageChanged,
}

pub fn run() {
    let minimized = std::env::args().any(|a| a == autostart::ARG_MINIMIZED);
    let (tx, rx) = async_channel::unbounded::<Cmd>();

    let show_tx = tx.clone();
    if !instance::acquire(move || {
        let _ = show_tx.try_send(Cmd::Show);
    }) {
        return;
    }
    let Some(store) = system::data_dir().and_then(|d| Store::new(d).ok()) else {
        return;
    };
    let data_dir = store.base().to_path_buf();
    update::clean_helpers(&data_dir);
    // An update downloaded earlier and never installed (the PC was shut down while the app
    // ran): install it now and come back the way we were started — only when the user turned
    // on "install automatically"; otherwise it waits for "Restart sekarang".
    // A version whose helper could not be started before is not tried again at every start:
    // it waits for "Restart sekarang" (or "Periksa update", which forgets the failure).
    if store.load_config().auto_install_app {
        let updates = Updates::new(&data_dir);
        if let Some(p) = updates.pending() {
            let version = p.info.version;
            if updates.failed_version().as_deref() != Some(version.as_str()) {
                let again = if minimized { Relaunch::Minimized } else { Relaunch::Window };
                if update::apply(&data_dir, again).is_ok() {
                    return;
                }
                updates.set_failed_version(Some(&version));
            }
        }
    }
    let engine_tx = tx.clone();
    // Started by autostart at login: let Windows settle before the first pass (it is cheap with
    // the stored hashes, but may still download a release).
    let first_pass_after = if minimized { FIRST_PASS_AT_LOGIN } else { Duration::ZERO };
    let engine = Arc::new(Engine::start(store, first_pass_after, move |events| {
        let _ = engine_tx.try_send(Cmd::Engine(events.to_vec()));
    }));

    gpui_platform::application().run(move |cx: &mut App| {
        // GPUI quits when the last window closes on Windows by default; the guard must keep
        // running in the tray until "Keluar".
        cx.set_quit_mode(QuitMode::Explicit);
        // AppUserModelID for notifications; also registers the display name for them.
        cx.set_app_identity(IDENTIFIER, PRODUCT_NAME);
        cx.set_global(Theme::new(cx.window_appearance()));
        cx.bind_keys([
            KeyBinding::new("tab", FocusNext, None),
            KeyBinding::new("shift-tab", FocusPrev, None),
            KeyBinding::new("escape", Dismiss, None),
        ]);

        let state = cx.new(|cx| AppState::new(engine.clone(), tx.clone(), cx));
        // Lives on the main thread for as long as the app runs (dropped on "Keluar").
        let tray = tray::build(tx.clone()).ok();
        let loop_state = state.clone();
        cx.spawn(async move |cx| {
            let mut tray = tray;
            // What the tray's update item shows; it is only touched when this changes.
            let mut tray_update: Option<String> = None;
            while let Ok(cmd) = rx.recv().await {
                let relaunch = match cmd {
                    Cmd::Quit => false,
                    Cmd::RestartForUpdate => true,
                    cmd => {
                        if let Some(t) = tray.as_ref() {
                            match &cmd {
                                Cmd::Engine(_) => {
                                    let now = engine.app_update();
                                    if now != tray_update {
                                        t.set_update(now.as_deref());
                                        tray_update = now;
                                    }
                                }
                                Cmd::LanguageChanged => {
                                    tray_update = engine.app_update();
                                    t.relabel(tray_update.as_deref())
                                }
                                _ => {}
                            }
                        }
                        cx.update(|cx| handle(cmd, &loop_state, cx));
                        continue;
                    }
                };
                // Let a guard pass that is writing game files finish first (at most a minute).
                for _ in 0..120 {
                    if !engine.busy() {
                        break;
                    }
                    cx.background_executor().timer(Duration::from_millis(500)).await;
                }
                // Quitting installs a waiting update only with "install automatically" on.
                let install = relaunch || engine.config().auto_install_app;
                if let Some(p) = engine.pending_update().filter(|_| install) {
                    let again = if relaunch { Relaunch::Window } else { Relaunch::No };
                    if let Err(e) = update::apply(&data_dir, again) {
                        // Quitting goes on; the next start must not retry it by itself.
                        if !relaunch {
                            Updates::new(&data_dir).set_failed_version(Some(&p.info.version));
                        }
                        if relaunch {
                            cx.update(|cx| loop_state.update(cx, |s, cx| s.show_error(e, cx)));
                            continue;
                        }
                    }
                } else if relaunch {
                    continue;
                }
                // Remove the icon first, or it lingers in the tray until hovered.
                drop(tray.take());
                cx.update(|cx| cx.quit());
                break;
            }
        })
        .detach();

        if !minimized {
            show_window(&state, cx);
        }
    });
}

fn handle(cmd: Cmd, state: &Entity<AppState>, cx: &mut App) {
    match cmd {
        Cmd::Show => show_window(state, cx),
        Cmd::CheckNow => state.update(cx, |s, cx| s.check_now(cx)),
        Cmd::Engine(events) => {
            notify(&events, cx);
            // Nothing to redraw while only the tray icon is there (most of the time); the
            // window reloads the engine state when it opens.
            if !cx.windows().is_empty() {
                state.update(cx, |s, cx| s.refresh(cx));
            }
        }
        // Handled by the command loop in `run`.
        Cmd::Quit | Cmd::RestartForUpdate | Cmd::LanguageChanged => {}
    }
}

fn notify(events: &[Event], cx: &mut App) {
    let restored = events.iter().filter(|e| matches!(e, Event::Restored { .. })).count();
    let installed = events.iter().filter(|e| matches!(e, Event::Installed { .. })).count();
    let title = match (restored, installed) {
        (0, 0) => return,
        (_, 0) => t!("notify.reinstalled"),
        _ => t!("notify.updated"),
    };
    let n = restored + installed;
    cx.show_system_notification(SystemNotification {
        tag: "patch-guard".into(),
        title: title.into(),
        body: tf!("notify.files_restored", n = n)
        .into(),
        actions: Vec::new(),
    });
}

/// Bring the window up, or create it. Closing it destroys it; the guard keeps running in the
/// tray until "Keluar".
fn show_window(state: &Entity<AppState>, cx: &mut App) {
    if let Some(w) = cx.windows().into_iter().next() {
        let _ = w.update(cx, |_, window, _| window.activate_window());
        return;
    }
    let bounds = Bounds::centered(None, size(px(900.), px(660.)), cx);
    let state = state.clone();
    let opened = cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions { title: Some(PRODUCT_NAME.into()), ..Default::default() }),
            window_min_size: Some(size(px(720.), px(520.))),
            window_background: if system::mica_supported() {
                WindowBackgroundAppearance::MicaBackdrop
            } else {
                WindowBackgroundAppearance::Opaque
            },
            app_id: Some(IDENTIFIER.into()),
            ..Default::default()
        },
        |window, cx| cx.new(|cx| Root::new(state, window, cx)),
    );
    if let Ok(w) = opened {
        let _ = w.update(cx, |_, window, _| window.activate_window());
    }
}

pub struct Root {
    state: Entity<AppState>,
    focus: FocusHandle,
    _subs: Vec<Subscription>,
}

impl Root {
    fn new(state: Entity<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let subs = vec![
            cx.observe(&state, |_, _, cx| cx.notify()),
            // Light/dark switch and accent colour: rebuild the palette.
            cx.observe_window_appearance(window, |_, window, cx| {
                cx.set_global(Theme::new(window.appearance()));
                cx.notify();
            }),
        ];
        // The engine may have finished a pass while the window was closed.
        state.update(cx, |s, cx| s.refresh(cx));
        Root { state, focus, _subs: subs }
    }

    fn nav_item(&self, page: Page, g: &'static str, label: &'static str, cx: &App) -> impl IntoElement {
        let t = theme(cx).clone();
        let selected = self.state.read(cx).page == page;
        let state = self.state.clone();
        div()
            // Not the label: it changes with the language.
            .id(("nav", page as usize))
            .relative()
            .flex()
            .items_center()
            .gap(px(12.))
            .h(px(36.))
            .px(px(12.))
            .rounded(px(4.))
            .border_1()
            .border_color(gpui::transparent_black())
            .when(selected, |el| el.bg(t.subtle_hover))
            .hover(move |s| s.bg(t.subtle_hover))
            .active(move |s| s.bg(t.subtle_pressed))
            .focusable()
            .tab_stop(true)
            .focus_visible(move |s| s.border_color(t.focus))
            .role(Role::Tab)
            .aria_label(label)
            .on_click(move |_, _, cx| state.update(cx, |s, cx| s.go(page, cx)))
            .when(selected, |el| {
                el.child(
                    div().absolute().left(px(-1.)).top(px(9.)).h(px(16.)).w(px(3.)).rounded_full().bg(t.accent),
                )
            })
            .child(icon(g, 16., t.foreground, cx))
            .child(label)
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = theme(cx).clone();
        let (page, error, confirm) = {
            let s = self.state.read(cx);
            (s.page, s.error.clone(), s.confirm.clone())
        };
        let content = match page {
            Page::Home => home::render(&self.state, cx),
            Page::Settings => settings::render(&self.state, cx),
            Page::History => history::render(&self.state, cx),
        };
        let dismiss_state = self.state.clone();

        div()
            .id("root")
            .track_focus(&self.focus)
            .on_action(cx.listener(|_, _: &FocusNext, window, cx| window.focus_next(cx)))
            .on_action(cx.listener(|_, _: &FocusPrev, window, cx| window.focus_prev(cx)))
            .on_action(cx.listener(|this, _: &Dismiss, _, cx| {
                this.state.update(cx, |s, cx| s.answer(false, cx))
            }))
            .relative()
            .size_full()
            .flex()
            .bg(t.background)
            .text_color(t.foreground)
            .font_family(t.font.clone())
            .text_size(px(14.))
            .line_height(px(20.))
            .role(Role::Application)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_none()
                    .gap(px(4.))
                    .p(px(8.))
                    .w(px(208.))
                    .h_full()
                    .id("nav")
                    .role(Role::TabList)
                    .child(
                        div()
                            .px(px(12.))
                            .pt(px(12.))
                            .pb(px(16.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(PRODUCT_NAME),
                    )
                    .child(self.nav_item(Page::Home, glyph::HOME, t!("common.home"), cx))
                    .child(self.nav_item(Page::Settings, glyph::SETTINGS, t!("common.settings"), cx))
                    .child(self.nav_item(Page::History, glyph::HISTORY, t!("common.history"), cx))
                    .child(div().flex_1())
                    .child(
                        div()
                            .px(px(12.))
                            .pb(px(8.))
                            .text_size(px(12.))
                            .line_height(px(16.))
                            .text_color(t.muted_fg)
                            .child({
                                let v = env!("CARGO_PKG_VERSION");
                                tf!("common.version", v = v)
                            }),
                    ),
            )
            .child(
                div()
                    .id("main")
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.))
                            .w_full()
                            .max_w(px(768.))
                            .mx_auto()
                            .px(px(24.))
                            .pt(px(20.))
                            .pb(px(24.))
                            .children(error.map(|e| {
                                error_bar(
                                    t!("app.error_title"),
                                    e,
                                    Rc::new(move |_, _, cx| {
                                        dismiss_state.update(cx, |s, cx| s.dismiss_error(cx))
                                    }),
                                    cx,
                                )
                            }))
                            .children(content),
                    ),
            )
            .children(confirm.map(|c| dialog(c.title, c.body, c.primary, &self.state, cx)))
    }
}

/// WinUI `ContentDialog`: a modal over a dimmed window.
fn dialog(
    title: SharedString,
    body: SharedString,
    primary: SharedString,
    state: &Entity<AppState>,
    cx: &App,
) -> impl IntoElement {
    let t = theme(cx).clone();
    let (yes, no) = (state.clone(), state.clone());
    div()
        .id("dialog-smoke")
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .bg(t.smoke)
        .occlude()
        .child(
            div()
                .flex()
                .flex_col()
                .w(px(448.))
                .rounded(px(8.))
                .border_1()
                .border_color(t.border)
                .bg(t.dialog)
                .shadow_lg()
                .overflow_hidden()
                .id("dialog")
                .role(Role::Dialog)
                .aria_label(title.clone())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .p(px(24.))
                        .child(
                            div()
                                .font_family(t.display_font.clone())
                                .text_size(px(20.))
                                .line_height(px(28.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(title),
                        )
                        .child(div().child(body)),
                )
                .child(
                    div()
                        .flex()
                        .gap(px(8.))
                        .p(px(24.))
                        .bg(t.dialog_footer)
                        .border_t_1()
                        .border_color(t.border)
                        .child(
                            Button::new("dialog-yes", primary)
                                .style(ButtonStyle::Accent)
                                .wide()
                                .on_click(move |_, _, cx| yes.update(cx, |s, cx| s.answer(true, cx))),
                        )
                        .child(
                            Button::new("dialog-no", t!("dialog.cancel"))
                                .wide()
                                .on_click(move |_, _, cx| no.update(cx, |s, cx| s.answer(false, cx))),
                        ),
                ),
        )
}
