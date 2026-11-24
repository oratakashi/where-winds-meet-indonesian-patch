//! History: everything the guard did, newest first, grouped by day. Kept across restarts
//! (`history.json` in the app's data folder).
use super::{clock, day};
use crate::fluent::{BODY, Button, ButtonStyle, caption, card, divider, glyph, page_title, subtitle, theme};
use crate::model::LogLine;
use crate::state::{AppState, Confirm};
use gpui::prelude::*;
use gpui::{AnyElement, App, Entity, div, px};
use std::rc::Rc;

/// Newest first, consecutive lines of the same day together.
fn by_day(log: &[LogLine]) -> Vec<(String, Vec<LogLine>)> {
    let mut days: Vec<(String, Vec<LogLine>)> = Vec::new();
    for line in log.iter().rev() {
        let d = day(line.at);
        match days.last_mut() {
            Some((last, lines)) if *last == d => lines.push(line.clone()),
            _ => days.push((d, vec![line.clone()])),
        }
    }
    days
}

pub fn render(state: &Entity<AppState>, cx: &mut App) -> Vec<AnyElement> {
    let s = state.read(cx);
    let last = match s.snap.last_check {
        Some(at) => {
            let c = clock(at);
            tf!("common.last_checked", c = c)
        }
        None => t!("history.not_checked_yet").to_string(),
    };
    let days = by_day(&s.snap.log);
    let st = state.clone();

    let mut out = vec![
        page_title(t!("common.history"), cx).into_any_element(),
        div()
            .flex()
            .items_center()
            .gap(px(12.))
            .child(caption(last, cx).flex_1())
            .child(
                Button::new("clear", t!("history.clear_history"))
                    .icon(glyph::DELETE)
                    .style(ButtonStyle::Subtle)
                    .small()
                    .disabled(days.is_empty())
                    .on_click(move |_, _, cx| {
                        st.update(cx, |s, cx| {
                            s.ask(
                                Confirm {
                                    title: t!("history.clear_all_history").into(),
                                    body: t!("history.cleared_history_cannot_be").into(),
                                    primary: t!("history.clear_confirm").into(),
                                    action: Rc::new(|s, cx| s.clear_history(cx)),
                                },
                                cx,
                            )
                        })
                    }),
            )
            .into_any_element(),
    ];

    if days.is_empty() {
        out.push(
            card(cx)
                .p(px(16.))
                .child(div().text_color(theme(cx).muted_fg).child(t!("history.nothing_has_happened_yet")))
                .into_any_element(),
        );
        return out;
    }
    let muted = theme(cx).muted_fg;
    for (d, lines) in days {
        out.push(subtitle(d).mt(px(8.)).into_any_element());
        let mut c = card(cx);
        for (i, l) in lines.into_iter().enumerate() {
            if i > 0 {
                c = c.child(divider(cx));
            }
            c = c.child(
                div()
                    .flex()
                    .gap(px(12.))
                    .px(px(16.))
                    .py(px(8.))
                    .text_size(px(BODY - 1.))
                    .line_height(px(18.))
                    .child(div().flex_none().font_family("Consolas").text_color(muted).child(clock(l.at)))
                    .child(div().flex_1().min_w_0().child(l.display())),
            );
        }
        out.push(c.into_any_element());
    }
    out
}
