//! Settings: autostart, translation and app updates, pause, and putting the original files back.
use crate::fluent::{Button, Toggle, caption, card, divider, page_title};
use crate::state::{AppState, Confirm};
use gpui::prelude::*;
use gpui::{AnyElement, App, Div, Entity, FontWeight, Window, div, px};
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
        page_title("Settings", cx).into_any_element(),
        card(cx)
            .child(setting(
                "autostart",
                "Jalankan saat Windows menyala",
                "Mulai di tray, tanpa membuka jendela.",
                autostart,
                move |v, _, cx| a.update(cx, |s, cx| s.set_autostart(v, cx)),
                cx,
            ))
            .child(divider(cx))
            .child(setting(
                "auto-update",
                "Perbarui terjemahan otomatis",
                "Pasang build terjemahan baru untuk versi game yang sama.",
                auto_update,
                move |v, _, cx| b.update(cx, |s, cx| s.set_auto_update(v, cx)),
                cx,
            ))
            .child(divider(cx))
            .child(setting(
                "paused",
                "Jeda penjaga",
                "Hanya memantau; tidak mengubah file game.",
                paused,
                move |v, _, cx| c.update(cx, |s, cx| s.set_paused(v, cx)),
                cx,
            ))
            .child(divider(cx))
            .child(
                row("Kembalikan file asli", "Batalkan patch di semua folder dan jeda penjaga.", cx).child(
                    Button::new("restore", "Kembalikan").disabled(no_dirs).on_click(move |_, _, cx| {
                        r.update(cx, |s, cx| {
                            s.ask(
                                Confirm {
                                    title: "Kembalikan file asli?".into(),
                                    body: "Kembalikan file asli game di semua folder?\n\nPenjaga \
                                           akan dijeda supaya patch tidak dipasang lagi."
                                        .into(),
                                    primary: "Kembalikan".into(),
                                    action: Rc::new(|s, cx| s.restore_originals(cx)),
                                },
                                cx,
                            )
                        })
                    }),
                ),
            )
            .into_any_element(),
        div()
            .pt(px(8.))
            .child(caption("Menutup jendela ini menyembunyikannya ke tray; penjaga tetap berjalan.", cx))
            .into_any_element(),
        card(cx)
            .child(setting(
                "auto-update-app",
                "Perbarui Patch Guard otomatis",
                "Unduh versi baru di latar belakang, lalu tawarkan restart untuk memasangnya.",
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
                            .child(
                                div()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(format!("Versi {}", env!("CARGO_PKG_VERSION"))),
                            )
                            .child(caption(
                                match (&ready, &note) {
                                    (Some(v), _) => format!("Versi {v} siap dipasang — buka Home untuk restart."),
                                    (None, Some(n)) => n.clone(),
                                    (None, None) => "Update diperiksa otomatis setiap 6 jam.".to_string(),
                                },
                                cx,
                            )),
                    )
                    .child(
                        Button::new("check-app-update", if app_checking { "Memeriksa…" } else { "Periksa update" })
                            .disabled(app_checking)
                            .on_click(move |_, _, cx| chk.update(cx, |s, cx| s.check_app_update(cx))),
                    ),
            )
            .into_any_element(),
    ]
}

fn row(title: &'static str, hint: &'static str, cx: &App) -> Div {
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
                .child(div().font_weight(FontWeight::MEDIUM).child(title))
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
