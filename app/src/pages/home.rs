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
    let mut out = vec![page_title(t!("common.home"), cx).into_any_element()];
    if let Some(v) = app_update {
        let restart = state.clone();
        out.push(
            info_bar(
                "app-update",
                tf!("home.patch_guard_v_is", v = v).into(),
                if s.snap.auto_install_app {
                    t!("home.downloaded_and_verified_patch")
                } else {
                    t!("home.downloaded_and_verified_manual")
                }
                .into(),
                t!("home.restart_now").into(),
                Rc::new(move |_, _, cx| restart.update(cx, |s, cx| s.restart_for_update(cx))),
                cx,
            )
            .into_any_element(),
        );
    }
    out.push(subtitle(t!("home.current_status")).into_any_element());
    out.push(hero(state, cx).into_any_element());

    let rescan = state.clone();
    let pick = state.clone();
    out.push(
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .mt(px(12.))
            .child(subtitle(t!("home.active_folders")).flex_1())
            .child(
                Button::new("rescan", t!("home.detect_again"))
                    .icon(glyph::REFRESH)
                    .style(ButtonStyle::Subtle)
                    .small()
                    .on_click(move |_, _, cx| rescan.update(cx, |s, cx| s.rescan(cx))),
            )
            .child(
                Button::new("pick", t!("home.choose_folder_manually"))
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
                .child(div().font_weight(FontWeight::MEDIUM).child(t!("home.game_not_found_automatically")))
                .child(div().text_color(theme(cx).muted_fg).child(t!("home.choose_the_where_winds")))
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
        (glyph::ADD, t.muted_fg, t!("home.not_set_up"), t!("home.no_game_folder_is"))
    } else if u.paused {
        (glyph::PAUSE, t.warning, t!("home.guard_paused"), t!("home.game_files_are_left"))
    } else if files().any(|f| f.status == Status::Error) {
        (glyph::ERROR, t.destructive, t!("home.something_needs_attention"), t!("home.see_history_for_details"))
    } else if files().any(|f| f.status == Status::Busy) {
        (glyph::SYNC, t.info, t!("home.waiting_for_game_files"), t!("home.files_are_being_written"))
    } else if files().any(|f| f.status == Status::Unknown) {
        (glyph::WARNING, t.warning, t!("home.waiting_for_a_new"), t!("home.this_game_file_version"))
    } else {
        (glyph::ACCEPT, t.success, t!("home.patch_guarded"), t!("home.reinstalled_automatically_if_overwritten"))
    };
    let last = u.last_check.map(|at| {
        let c = clock(at);
        tf!("common.last_checked", c = c)
    });
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
            Button::new("check", if checking { t!("common.checking") } else { t!("common.check_now") })
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
                    .child(subtitle(t!("home.games_detected")))
                    .child(caption(t!("home.found_on_this_pc"), cx)),
            )
            .when(many, |el| {
                let st = state.clone();
                el.child(
                    Button::new("guard-all", t!("home.guard_all"))
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
                    Button::new(("guard", i), t!("home.guard"))
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
        Status::Patched => (Tone::Success, t!("home.status.installed")),
        Status::Original => (Tone::Warning, t!("home.status.original")),
        Status::Unknown => (Tone::Warning, t!("home.status.unknown")),
        Status::Missing => (Tone::Muted, t!("home.status.missing")),
        Status::Busy => (Tone::Info, t!("home.status.waiting")),
        Status::Error => (Tone::Error, t!("home.status.failed")),
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
                el.child(Button::new(("force", i), t!("home.force_install")).small().on_click(move |_, _, cx| {
                    let path = force_path.clone();
                    force_state.update(cx, |s, cx| {
                        s.ask(
                            Confirm {
                                title: t!("home.force_install_title").into(),
                                body: t!("home.force_install_body").into(),
                                primary: t!("home.force_install").into(),
                                action: Rc::new(move |s, cx| s.force_install(path.clone(), cx)),
                            },
                            cx,
                        )
                    })
                }))
            })
            .child(
                Button::new(("remove", i), t!("home.stop_guarding"))
                    .style(ButtonStyle::Subtle)
                    .small()
                    .on_click(move |_, _, cx| {
                        let p = path.clone();
                        remove_state.update(cx, |s, cx| {
                            s.ask(
                                Confirm {
                                    title: t!("home.stop_guarding_title").into(),
                                    body: t!("home.stop_guarding_body").into(),
                                    primary: t!("home.stop_guarding").into(),
                                    action: Rc::new(move |s, cx| s.remove_folder(p.clone(), cx)),
                                },
                                cx,
                            )
                        })
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
