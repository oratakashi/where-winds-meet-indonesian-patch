//! Home: the current status, and the folders being guarded ("active directories") together
//! with the game folders found on this machine that are not guarded yet.
use crate::api::{self, DirReport, FileReport, Found, Status};
use crate::app::{Ctx, PageTitle, client_title, clock, confirm, profile_title};
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardContent};
use leptos::prelude::*;

#[component]
pub fn Home(ctx: Ctx) -> impl IntoView {
    let pick = move |_| {
        ctx.run(async move {
            match api::add_folder().await? {
                Some(0) => Err("Folder itu sudah dijaga.".to_string()),
                _ => Ok(()),
            }
        })
    };
    // Found on this machine, not guarded yet.
    let pending = move || ctx.found.with(|f| f.iter().filter(|x| !x.guarded).cloned().collect::<Vec<_>>());
    // Picking a folder by hand is the fallback: prominent only when nothing was detected.
    let nothing_found = move || ctx.found.with(|f| f.is_empty());

    view! {
        <PageTitle text="Home" />

        <h2 class="text-sm font-semibold">"Status saat ini"</h2>
        <Hero ctx />

        <div class="flex gap-2 items-center mt-3">
            <h2 class="text-sm font-semibold grow">"Folder aktif"</h2>
            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=move |_| ctx.rescan()>
                <span class="icon">"\u{E72C}"</span>
                "Deteksi ulang"
            </Button>
            {move || {
                let variant = if nothing_found() { ButtonVariant::Default } else { ButtonVariant::Outline };
                view! {
                    <Button variant=variant size=ButtonSize::Sm on:click=pick>
                        <span class="icon">"\u{E8B7}"</span>
                        "Pilih folder manual"
                    </Button>
                }
            }}
        </div>

        {move || {
            let dirs = ctx.ui.with(|u| u.dirs.clone());
            dirs.into_iter().map(|d| view! { <DirCard ctx dir=d /> }).collect_view()
        }}

        {move || {
            let pending = pending();
            (!pending.is_empty()).then(|| view! { <Detected ctx pending /> })
        }}

        {move || {
            (ctx.ui.with(|u| u.dirs.is_empty()) && nothing_found())
                .then(|| {
                    view! {
                        <Card>
                            <CardContent class="flex flex-col gap-1">
                                <p class="font-medium">"Game tidak ditemukan otomatis"</p>
                                <p class="text-muted-foreground">
                                    "Pilih folder instalasi Where Winds Meet secara manual (folder Steam, launcher, atau folder game-nya). Folder bahasa di dalamnya dicari otomatis."
                                </p>
                            </CardContent>
                        </Card>
                    }
                })
        }}
    }
}

/// Status summary, like the banner at the top of Windows Update.
#[component]
fn Hero(ctx: Ctx) -> impl IntoView {
    move || {
        let (glyph, tone, title, sub) = ctx.ui.with(|u| {
            let files = || u.dirs.iter().flat_map(|d| &d.files);
            if u.dirs.is_empty() {
                ("\u{E710}", "text-muted-foreground", "Belum diatur", "Belum ada folder game yang dijaga.")
            } else if u.paused {
                ("\u{E769}", "text-warning", "Penjaga dijeda", "File game tidak diubah sampai dilanjutkan.")
            } else if files().any(|f| f.status == Status::Error) {
                ("\u{EA39}", "text-destructive", "Ada masalah", "Lihat History untuk detailnya.")
            } else if files().any(|f| f.status == Status::Busy) {
                ("\u{E895}", "text-info", "Menunggu file game", "File sedang ditulis atau dipakai; dicoba lagi otomatis.")
            } else if files().any(|f| f.status == Status::Unknown) {
                ("\u{E7BA}", "text-warning", "Menunggu rilis baru", "Versi file game belum punya patch yang cocok.")
            } else {
                ("\u{E73E}", "text-success", "Patch terjaga", "Akan dipasang ulang otomatis jika ditimpa.")
            }
        });
        let last = ctx.ui.with(|u| u.last_check).map(|at| format!("Terakhir diperiksa {}", clock(at)));
        view! {
            <Card>
                <CardContent class="flex gap-4 items-center">
                    <span class=format!("icon text-[32px] {tone}")>{glyph}</span>
                    <div class="min-w-0 grow">
                        <p class="text-base font-semibold">{title}</p>
                        <p class="text-muted-foreground">{sub}</p>
                        {last.map(|l| view! { <p class="text-xs text-muted-foreground">{l}</p> })}
                    </div>
                    <Button
                        variant=ButtonVariant::Outline
                        attr:disabled=move || ctx.ui.with(|u| u.checking || u.dirs.is_empty())
                        on:click=move |_| ctx.run(api::check_now())
                    >
                        {move || if ctx.ui.with(|u| u.checking) { "Memeriksa…" } else { "Periksa sekarang" }}
                    </Button>
                </CardContent>
            </Card>
        }
    }
}

/// Game folders found on this machine that are not guarded yet.
#[component]
fn Detected(ctx: Ctx, pending: Vec<Found>) -> impl IntoView {
    let all: Vec<String> = pending.iter().map(|f| f.path.clone()).collect();
    let many = all.len() > 1;
    let guard = move |paths: Vec<String>| {
        ctx.run(async move { api::add_detected(&paths).await.map(|_| ()) });
    };
    view! {
        <Card class="gap-0 pb-0">
            <CardContent class="flex gap-3 items-center pb-3">
                <div class="min-w-0 grow">
                    <p class="font-semibold">"Game terdeteksi"</p>
                    <p class="text-xs text-muted-foreground">"Ditemukan di komputer ini, belum dijaga."</p>
                </div>
                {many
                    .then(|| {
                        view! {
                            <Button size=ButtonSize::Sm on:click=move |_| guard(all.clone())>
                                "Jaga semua"
                            </Button>
                        }
                    })}
            </CardContent>
            <div class="flex flex-col border-t divide-y">
                {pending
                    .into_iter()
                    .map(|f| {
                        let path = f.path.clone();
                        let variant = if many { ButtonVariant::Outline } else { ButtonVariant::Default };
                        view! {
                            <div class="flex gap-3 items-center py-2.5 px-4">
                                <div class="min-w-0 grow">
                                    <p class="flex gap-2 items-center font-medium">
                                        {profile_title(f.profile)}
                                        <Badge variant=BadgeVariant::Muted>{client_title(f.client)}</Badge>
                                    </p>
                                    <p class="text-xs break-all select-text text-muted-foreground">{f.path}</p>
                                </div>
                                <Button
                                    variant=variant
                                    size=ButtonSize::Sm
                                    on:click=move |_| guard(vec![path.clone()])
                                >
                                    "Jaga"
                                </Button>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </Card>
    }
}

fn status_badge(s: Status) -> (BadgeVariant, &'static str) {
    match s {
        Status::Patched => (BadgeVariant::Success, "Terpasang"),
        Status::Original => (BadgeVariant::Warning, "File asli"),
        Status::Unknown => (BadgeVariant::Warning, "Tidak dikenal"),
        Status::Missing => (BadgeVariant::Muted, "Tidak ada"),
        Status::Busy => (BadgeVariant::Info, "Menunggu"),
        Status::Error => (BadgeVariant::Destructive, "Gagal"),
    }
}

#[component]
fn DirCard(ctx: Ctx, dir: DirReport) -> impl IntoView {
    let has_unknown = dir.files.iter().any(|f| f.status == Status::Unknown);
    let path = StoredValue::new(dir.path.clone());

    let force = move |_| {
        if confirm(
            "Pasang rilis terbaru di folder ini walau versi file game-nya tidak dikenali?\n\n\
             Lakukan ini hanya kalau game sudah versi terbaru (misalnya file itu patch lama \
             yang dipasang manual). File yang ada sekarang disimpan sebagai cadangan.",
        ) {
            ctx.run(async move { api::force_install(&path.get_value()).await });
        }
    };
    let remove = move |_| ctx.run(async move { api::remove_folder(&path.get_value()).await });

    view! {
        <Card>
            <CardContent class="flex flex-col gap-0.5">
                <p class="flex gap-2 items-center font-semibold">
                    {profile_title(dir.profile)}
                    <Badge variant=BadgeVariant::Muted>{client_title(dir.client)}</Badge>
                </p>
                <p class="text-xs break-all select-text text-muted-foreground">{dir.path.clone()}</p>
            </CardContent>
            <div class="flex flex-col border-t divide-y">
                {dir.files.into_iter().map(|f| view! { <FileRow file=f /> }).collect_view()}
            </div>
            <CardContent class="flex flex-wrap gap-2 pt-1">
                {has_unknown
                    .then(|| {
                        view! {
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm on:click=force>
                                "Pasang paksa"
                            </Button>
                        }
                    })}
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=remove>
                    "Berhenti menjaga"
                </Button>
            </CardContent>
        </Card>
    }
}

#[component]
fn FileRow(file: FileReport) -> impl IntoView {
    let (variant, label) = status_badge(file.status);
    view! {
        <div class="flex gap-3 justify-between items-center py-2.5 px-4">
            <div class="min-w-0">
                <p class="font-mono break-all text-[13px]">{file.name}</p>
                {(!file.detail.is_empty())
                    .then(|| view! { <p class="text-xs text-muted-foreground">{file.detail}</p> })}
            </div>
            <Badge variant=variant class="shrink-0">{label}</Badge>
        </div>
    }
}
