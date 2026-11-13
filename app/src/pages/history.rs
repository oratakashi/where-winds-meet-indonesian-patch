//! History: everything the guard did, newest first, grouped by day. Kept across restarts
//! (`history.json` in the app's data folder).
use crate::api::{self, LogLine};
use crate::app::{Ctx, PageTitle, clock, confirm, day};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardContent};
use leptos::prelude::*;

/// Newest first, consecutive lines of the same day together.
fn by_day(log: Vec<LogLine>) -> Vec<(String, Vec<LogLine>)> {
    let mut days: Vec<(String, Vec<LogLine>)> = Vec::new();
    for line in log.into_iter().rev() {
        let d = day(line.at);
        match days.last_mut() {
            Some((last, lines)) if *last == d => lines.push(line),
            _ => days.push((d, vec![line])),
        }
    }
    days
}

#[component]
pub fn History(ctx: Ctx) -> impl IntoView {
    let clear = move |_| {
        if confirm("Hapus semua riwayat?") {
            ctx.run(api::clear_history());
        }
    };
    view! {
        <PageTitle text="History" />
        <div class="flex gap-3 items-center">
            <p class="text-xs text-muted-foreground grow">
                {move || match ctx.ui.with(|u| u.last_check) {
                    Some(at) => format!("Terakhir diperiksa {}", clock(at)),
                    None => "Belum diperiksa".to_string(),
                }}
            </p>
            <Button
                variant=ButtonVariant::Ghost
                size=ButtonSize::Sm
                attr:disabled=move || ctx.ui.with(|u| u.log.is_empty())
                on:click=clear
            >
                <span class="icon">"\u{E74D}"</span>
                "Hapus riwayat"
            </Button>
        </div>
        {move || {
            let days = by_day(ctx.ui.with(|u| u.log.clone()));
            if days.is_empty() {
                return view! {
                    <Card>
                        <CardContent>
                            <p class="text-muted-foreground">"Belum ada kejadian."</p>
                        </CardContent>
                    </Card>
                }
                    .into_any();
            }
            days.into_iter()
                .map(|(d, lines)| {
                    view! {
                        <h2 class="mt-2 text-sm font-semibold">{d}</h2>
                        <Card class="gap-0 py-0">
                            <div class="flex flex-col divide-y">
                                {lines
                                    .into_iter()
                                    .map(|l| {
                                        view! {
                                            <p class="flex gap-3 py-2 px-4 select-text text-[13px]">
                                                <span class="font-mono text-muted-foreground shrink-0">
                                                    {clock(l.at)}
                                                </span>
                                                <span class="min-w-0 break-words">{l.text}</span>
                                            </p>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </Card>
                    }
                })
                .collect_view()
                .into_any()
        }}
    }
}
