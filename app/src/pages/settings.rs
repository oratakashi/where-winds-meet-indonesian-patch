use crate::api;
use crate::app::{Ctx, PageTitle, confirm};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::Card;
use crate::components::ui::switch::Switch;
use leptos::prelude::*;

#[component]
pub fn Settings(ctx: Ctx) -> impl IntoView {
    let restore = move |_| {
        if confirm(
            "Kembalikan file asli game di semua folder?\n\n\
             Penjaga akan dijeda supaya patch tidak dipasang lagi.",
        ) {
            ctx.run(api::restore_originals());
        }
    };
    view! {
        <PageTitle text="Settings" />
        <Card class="gap-0 py-0">
            <Setting
                title="Jalankan saat Windows menyala"
                hint="Mulai di tray, tanpa membuka jendela."
                checked=Signal::derive(move || ctx.ui.with(|u| u.autostart))
                on_change=move |v| ctx.run(api::set_autostart(v))
            />
            <Setting
                title="Perbarui terjemahan otomatis"
                hint="Pasang build terjemahan baru untuk versi game yang sama."
                checked=Signal::derive(move || ctx.ui.with(|u| u.auto_update))
                on_change=move |v| ctx.run(api::set_auto_update(v))
            />
            <Setting
                title="Jeda penjaga"
                hint="Hanya memantau; tidak mengubah file game."
                checked=Signal::derive(move || ctx.ui.with(|u| u.paused))
                on_change=move |v| ctx.run(api::set_paused(v))
            />
            <div class="flex gap-4 justify-between items-center py-3 px-4">
                <div>
                    <p class="font-medium">"Kembalikan file asli"</p>
                    <p class="text-xs text-muted-foreground">
                        "Batalkan patch di semua folder dan jeda penjaga."
                    </p>
                </div>
                <Button
                    variant=ButtonVariant::Outline
                    attr:disabled=move || ctx.ui.with(|u| u.dirs.is_empty())
                    on:click=restore
                >
                    "Kembalikan"
                </Button>
            </div>
        </Card>
        <p class="pt-2 text-xs text-muted-foreground">
            "Menutup jendela ini menyembunyikannya ke tray; penjaga tetap berjalan."
        </p>
    }
}

#[component]
fn Setting(
    title: &'static str,
    hint: &'static str,
    checked: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <div class="flex gap-4 justify-between items-center py-3 px-4 border-b">
            <div>
                <p class="font-medium">{title}</p>
                <p class="text-xs text-muted-foreground">{hint}</p>
            </div>
            <Switch checked=checked on_change=on_change aria_label=title />
        </div>
    }
}
