//! Home: the current status, and the folders being guarded ("active directories") together
//! with the game folders found on this machine that are not guarded yet.
use super::{client_title, clock, profile_title};
use crate::fluent::{
    BODY, Button, ButtonStyle, Tone, badge, caption, card, divider, glyph, icon, info_bar,
    page_title, subtitle, theme,
};
use crate::model::{DirReport, FileReport, Status};
use crate::state::{AppState, Confirm, Found};
use gpui::prelude::*;
use gpui::{App, AnyElement, Div, Entity, FontWeight, SharedString, div, px};
use std::path::PathBuf;
use std::rc::Rc;

pub fn render(state: &Entity<AppState>, cx: &mut App) -> Vec<AnyElement> {
    let s = state.read(cx);
    let dirs = s.snap.dirs.clone();
    let pending: Vec<Found> = s.found.iter().filter(|f| !f.guarded).cloned().collect();
    // Picking a folder by hand is the fallback: prominent only when nothing was detected.
    let nothing_found = s.found.is_empty();
    let no_dirs = dirs.is_empty();

    let app_update = s.snap.app_update.clone();
    let mut out = vec![page_title("Home", cx).into_any_element()];
    if let Some(v) = app_update {
        let restart = state.clone();
        out.push(
            info_bar(
                "app-update",
                format!("Patch Guard v{v} siap dipasang").into(),
                "Sudah diunduh dan diverifikasi. Patch Guard akan ditutup sebentar lalu dibuka \
                 lagi; kalau tidak sekarang, update dipasang saat keluar atau saat Windows \
                 menyala berikutnya."
                    .into(),
                "Restart sekarang".into(),
                Rc::new(move |_, _, cx| restart.update(cx, |s, cx| s.restart_for_update(cx))),
                cx,
            )
            .into_any_element(),
        );
    }
    out.push(subtitle("Status saat ini").into_any_element());
    out.push(hero(state, cx).into_any_element());

    let rescan = state.clone();
    let pick = state.clone();
    out.push(
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .mt(px(12.))
            .child(subtitle("Folder aktif").flex_1())
            .child(
                Button::new("rescan", "Deteksi ulang")
                    .icon(glyph::REFRESH)
                    .style(ButtonStyle::Subtle)
                    .small()
                    .on_click(move |_, _, cx| rescan.update(cx, |s, cx| s.rescan(cx))),
            )
            .child(
                Button::new("pick", "Pilih folder manual")
                    .icon(glyph::FOLDER_OPEN)
                    .style(if nothing_found { ButtonStyle::Accent } else { ButtonStyle::Standard })
                    .small()
                    .on_click(move |_, _, cx| pick.update(cx, |s, cx| s.add_folder(cx))),
            )
            .into_any_element(),
    );

    for (i, d) in dirs.into_iter().enumerate() {
        out.push(dir_card(i, d, state, cx).into_any_element());
    }
    if !pending.is_empty() {
        out.push(detected(pending, state, cx).into_any_element());
    }
    if no_dirs && nothing_found {
        out.push(
            card(cx)
                .gap(px(4.))
                .p(px(16.))
                .child(div().font_weight(FontWeight::MEDIUM).child("Game tidak ditemukan otomatis"))
                .child(div().text_color(theme(cx).muted_fg).child(
                    "Pilih folder instalasi Where Winds Meet secara manual (folder Steam, launcher, \
                     atau folder game-nya). Folder bahasa di dalamnya dicari otomatis.",
                ))
                .into_any_element(),
        );
    }
    out
}

/// Status summary, like the banner at the top of Windows Update.
fn hero(state: &Entity<AppState>, cx: &App) -> Div {
    let t = theme(cx).clone();
    let s = state.read(cx);
    let u = &s.snap;
    let files = || u.dirs.iter().flat_map(|d| &d.files);
    let (g, tone, title, sub) = if u.dirs.is_empty() {
        (glyph::ADD, t.muted_fg, "Belum diatur", "Belum ada folder game yang dijaga.")
    } else if u.paused {
        (glyph::PAUSE, t.warning, "Penjaga dijeda", "File game tidak diubah sampai dilanjutkan.")
    } else if files().any(|f| f.status == Status::Error) {
        (glyph::ERROR, t.destructive, "Ada masalah", "Lihat History untuk detailnya.")
    } else if files().any(|f| f.status == Status::Busy) {
        (glyph::SYNC, t.info, "Menunggu file game", "File sedang ditulis atau dipakai; dicoba lagi otomatis.")
    } else if files().any(|f| f.status == Status::Unknown) {
        (glyph::WARNING, t.warning, "Menunggu rilis baru", "Versi file game belum punya patch yang cocok.")
    } else {
        (glyph::ACCEPT, t.success, "Patch terjaga", "Akan dipasang ulang otomatis jika ditimpa.")
    };
    let last = u.last_check.map(|at| format!("Terakhir diperiksa {}", clock(at)));
    let checking = u.checking;
    let disabled = u.checking || u.dirs.is_empty();
    let check = state.clone();
    card(cx)
        .flex_row()
        .items_center()
        .gap(px(16.))
        .p(px(16.))
        .child(icon(g, 32., tone, cx))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .child(div().text_size(px(16.)).line_height(px(22.)).font_weight(FontWeight::SEMIBOLD).child(title))
                .child(div().text_color(t.muted_fg).child(sub))
                .children(last.map(|l| caption(l, cx))),
        )
        .child(
            Button::new("check", if checking { "Memeriksa…" } else { "Periksa sekarang" })
                .disabled(disabled)
                .on_click(move |_, _, cx| check.update(cx, |s, cx| s.check_now(cx))),
        )
}

/// Game folders found on this machine that are not guarded yet.
fn detected(pending: Vec<Found>, state: &Entity<AppState>, cx: &App) -> Div {
    let all: Vec<PathBuf> = pending.iter().map(|f| f.path.clone()).collect();
    let many = all.len() > 1;
    let mut c = card(cx).child(
        div()
            .flex()
            .items_center()
            .gap(px(12.))
            .px(px(16.))
            .pt(px(16.))
            .pb(px(12.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .child(subtitle("Game terdeteksi"))
                    .child(caption("Ditemukan di komputer ini, belum dijaga.", cx)),
            )
            .when(many, |el| {
                let st = state.clone();
                el.child(
                    Button::new("guard-all", "Jaga semua")
                        .style(ButtonStyle::Accent)
                        .small()
                        .on_click(move |_, _, cx| {
                            let all = all.clone();
                            st.update(cx, |s, cx| s.add_detected(all, cx))
                        }),
                )
            }),
    );
    for (i, f) in pending.into_iter().enumerate() {
        let st = state.clone();
        let path = f.path.clone();
        c = c.child(divider(cx)).child(
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .px(px(16.))
                .py(px(10.))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.))
                                .font_weight(FontWeight::MEDIUM)
                                .child(profile_title(f.profile))
                                .child(badge(client_title(f.client), Tone::Muted, cx)),
                        )
                        .child(caption(f.path.display().to_string(), cx)),
                )
                .child(
                    Button::new(("guard", i), "Jaga")
                        .style(if many { ButtonStyle::Standard } else { ButtonStyle::Accent })
                        .small()
                        .on_click(move |_, _, cx| {
                            let p = vec![path.clone()];
                            st.update(cx, |s, cx| s.add_detected(p, cx))
                        }),
                ),
        );
    }
    c
}

fn status_badge(s: Status) -> (Tone, &'static str) {
    match s {
        Status::Patched => (Tone::Success, "Terpasang"),
        Status::Original => (Tone::Warning, "File asli"),
        Status::Unknown => (Tone::Warning, "Tidak dikenal"),
        Status::Missing => (Tone::Muted, "Tidak ada"),
        Status::Busy => (Tone::Info, "Menunggu"),
        Status::Error => (Tone::Error, "Gagal"),
    }
}

fn dir_card(i: usize, dir: DirReport, state: &Entity<AppState>, cx: &App) -> Div {
    let has_unknown = dir.files.iter().any(|f| f.status == Status::Unknown);
    let path = PathBuf::from(&dir.path);
    let mut c = card(cx).child(
        div()
            .flex()
            .flex_col()
            .gap(px(2.))
            .px(px(16.))
            .pt(px(16.))
            .pb(px(12.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(profile_title(dir.profile))
                    .child(badge(client_title(dir.client), Tone::Muted, cx)),
            )
            .child(caption(dir.path.clone(), cx)),
    );
    for f in dir.files {
        c = c.child(divider(cx)).child(file_row(f, cx));
    }
    let force_state = state.clone();
    let force_path = path.clone();
    let remove_state = state.clone();
    c.child(divider(cx)).child(
        div()
            .flex()
            .flex_wrap()
            .gap(px(8.))
            .px(px(16.))
            .py(px(12.))
            .when(has_unknown, |el| {
                el.child(Button::new(("force", i), "Pasang paksa").small().on_click(move |_, _, cx| {
                    let path = force_path.clone();
                    force_state.update(cx, |s, cx| {
                        s.ask(
                            Confirm {
                                title: "Pasang paksa?".into(),
                                body: "Pasang rilis terbaru di folder ini walau versi file game-nya \
                                       tidak dikenali?\n\nLakukan ini hanya kalau game sudah versi \
                                       terbaru (misalnya file itu patch lama yang dipasang manual). \
                                       File yang ada sekarang disimpan sebagai cadangan."
                                    .into(),
                                primary: "Pasang paksa".into(),
                                action: Rc::new(move |s, cx| s.force_install(path.clone(), cx)),
                            },
                            cx,
                        )
                    })
                }))
            })
            .child(
                Button::new(("remove", i), "Berhenti menjaga")
                    .style(ButtonStyle::Subtle)
                    .small()
                    .on_click(move |_, _, cx| {
                        let p = path.clone();
                        remove_state.update(cx, |s, cx| s.remove_folder(p, cx))
                    }),
            ),
    )
}

fn file_row(file: FileReport, cx: &App) -> Div {
    let (tone, label) = status_badge(file.status);
    let detail: Option<SharedString> = (!file.detail.is_empty()).then(|| file.detail.into());
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(12.))
        .px(px(16.))
        .py(px(10.))
        .child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .child(div().font_family("Consolas").text_size(px(BODY - 1.)).child(file.name))
                .children(detail.map(|d| caption(d, cx))),
        )
        .child(badge(label, tone, cx))
}
