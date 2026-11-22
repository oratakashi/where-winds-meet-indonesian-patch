//! Settings: autostart, auto-update, pause, and putting the original files back.
use crate::fluent::{Button, Toggle, caption, card, divider, page_title};
use crate::state::{AppState, Confirm};
use gpui::prelude::*;
use gpui::{AnyElement, App, Div, Entity, FontWeight, Window, div, px};
use std::rc::Rc;

pub fn render(state: &Entity<AppState>, cx: &mut App) -> Vec<AnyElement> {
    let s = state.read(cx);
    let (autostart, auto_update, paused) = (s.autostart, s.snap.auto_update, s.snap.paused);
    let no_dirs = s.snap.dirs.is_empty();

    let (a, b, c, r) = (state.clone(), state.clone(), state.clone(), state.clone());
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
