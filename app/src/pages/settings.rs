//! Settings: language, autostart, translation and app updates, pause, and putting the
//! original files back.
use crate::fluent::{Button, ButtonStyle, Toggle, caption, card, divider, page_title};
use crate::i18n::{self, Lang};
use crate::state::{AppState, Confirm};
use gpui::prelude::*;
use gpui::{AnyElement, App, Div, Entity, FontWeight, SharedString, Window, div, px};
use std::rc::Rc;

pub fn render(state: &Entity<AppState>, cx: &mut App) -> Vec<AnyElement> {
    let s = state.read(cx);
    let (autostart, auto_update, paused) = (s.autostart, s.snap.auto_update, s.snap.paused);
    let no_dirs = s.snap.dirs.is_empty();
    let auto_update_app = s.snap.auto_update_app;
    let app_checking = s.snap.app_update_checking;
    let ready = s.snap.app_update.clone();
    let note = s.snap.app_update_note.clone();

    let (a, b, c, r) = (state.clone(), state.clone(), state.clone(), state.clone());
    let (u, chk) = (state.clone(), state.clone());
    vec![
        page_title(t!("common.settings"), cx).into_any_element(),
        card(cx).child(language_row(state, cx)).into_any_element(),
        card(cx)
            .child(setting(
                "autostart",
                t!("settings.run_when_windows_starts"),
                t!("settings.start_in_the_tray"),
                autostart,
                move |v, _, cx| a.update(cx, |s, cx| s.set_autostart(v, cx)),
                cx,
            ))
            .child(divider(cx))
            .child(setting(
                "auto-update",
                t!("settings.update_the_translation_automatically"),
                t!("settings.install_new_translation_builds"),
                auto_update,
                move |v, _, cx| b.update(cx, |s, cx| s.set_auto_update(v, cx)),
                cx,
            ))
            .child(divider(cx))
            .child(setting(
                "paused",
                t!("settings.pause_the_guard"),
                t!("settings.only_watch_never_change"),
                paused,
                move |v, _, cx| c.update(cx, |s, cx| s.set_paused(v, cx)),
                cx,
            ))
            .child(divider(cx))
            .child(
                row(
                    t!("settings.restore.row"),
                    t!("settings.restore.hint"),
                    cx,
                )
                .child(
                    Button::new("restore", t!("settings.restore.button")).disabled(no_dirs).on_click(
                        move |_, _, cx| {
                            r.update(cx, |s, cx| {
                                s.ask(
                                    Confirm {
                                        title: t!("settings.restore.title").into(),
                                        body: t!("settings.restore.body").into(),
                                        primary: t!("settings.restore.button").into(),
                                        action: Rc::new(|s, cx| s.restore_originals(cx)),
                                    },
                                    cx,
                                )
                            })
                        },
                    ),
                ),
            )
            .into_any_element(),
        div()
            .pt(px(8.))
            .child(caption(
                t!("settings.closing_this_window_hides"),
                cx,
            ))
            .into_any_element(),
        card(cx)
            .child(setting(
                "auto-update-app",
                t!("settings.update_patch_guard_automatically"),
                t!("settings.download_new_versions_in"),
                auto_update_app,
                move |v, _, cx| u.update(cx, |s, cx| s.set_auto_update_app(v, cx)),
                cx,
            ))
            .child(divider(cx))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(16.))
                    .px(px(16.))
                    .py(px(12.))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .min_w_0()
                            .child(div().font_weight(FontWeight::MEDIUM).child({
                                let v = env!("CARGO_PKG_VERSION");
                                tf!("common.version", v = v)
                            }))
                            .child(caption(
                                match (&ready, &note) {
                                    (Some(v), _) => tf!("settings.version_is_ready_to", v = v),
                                    (None, Some(n)) => n.text(),
                                    (None, None) => t!("settings.updates_are_checked_automatically").to_string(),
                                },
                                cx,
                            )),
                    )
                    .child(
                        Button::new(
                            "check-app-update",
                            if app_checking {
                                t!("common.checking")
                            } else {
                                t!("settings.check_for_updates")
                            },
                        )
                        .disabled(app_checking)
                        .on_click(move |_, _, cx| chk.update(cx, |s, cx| s.check_app_update(cx))),
                    ),
            )
            .into_any_element(),
    ]
}

/// "Bahasa / Language": one button per language, the current one highlighted. The label is
/// bilingual so it can be found whichever language is on.
fn language_row(state: &Entity<AppState>, cx: &App) -> Div {
    let current = i18n::get();
    let mut buttons = div().flex().gap(px(8.)).flex_none();
    for lang in Lang::ALL {
        let st = state.clone();
        buttons = buttons.child(
            Button::new(("lang", lang as usize), i18n::text_in(lang, "lang.name"))
                .style(if lang == current { ButtonStyle::Accent } else { ButtonStyle::Standard })
                .small()
                .on_click(move |_, _, cx| st.update(cx, |s, cx| s.set_language(lang, cx))),
        );
    }
    row(
        t!("settings.language.title"),
        t!("settings.language.hint"),
        cx,
    )
    .child(buttons)
}

fn row(title: impl Into<SharedString>, hint: impl Into<SharedString>, cx: &App) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(16.))
        .px(px(16.))
        .py(px(12.))
        .child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .child(div().font_weight(FontWeight::MEDIUM).child(title.into()))
                .child(caption(hint, cx)),
        )
}

fn setting(
    id: &'static str,
    title: &'static str,
    hint: &'static str,
    on: bool,
    f: impl Fn(bool, &mut Window, &mut App) + 'static,
    cx: &App,
) -> Div {
    row(title, hint, cx).child(Toggle::new(id, title, on).on_change(f))
}
