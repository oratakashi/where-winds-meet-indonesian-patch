use crate::api::{self, DirReport, FileReport, Profile, Status, Ui};
use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::switch::Switch;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::future::Future;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn confirm(message: &str) -> bool;
}

/// Shared by every part of the window.
#[derive(Clone, Copy)]
struct Ctx {
    ui: RwSignal<Ui>,
    error: RwSignal<Option<String>>,
}

impl Ctx {
    fn refresh(self) {
        spawn_local(async move {
            match api::get_status().await {
                Ok(u) => self.ui.set(u),
                Err(e) => self.error.set(Some(e)),
            }
        });
    }

    /// Run a backend command, show its error if any, then reload the status.
    fn run(self, fut: impl Future<Output = Result<(), String>> + 'static) {
        spawn_local(async move {
            match fut.await {
                Ok(()) => self.error.set(None),
                Err(e) => self.error.set(Some(e)),
            }
            self.refresh();
        });
    }
}

#[component]
pub fn App() -> impl IntoView {
    let ctx = Ctx { ui: RwSignal::new(Ui::default()), error: RwSignal::new(None) };
    ctx.refresh();
    spawn_local(api::on_status(move || ctx.refresh()));

    let add = move |_| {
        ctx.run(async move {
            match api::add_folder().await? {
                Some(0) => Err("Folder itu sudah dijaga.".to_string()),
                _ => Ok(()),
            }
        })
    };

    view! {
        <main class="flex flex-col gap-4 p-5 mx-auto max-w-2xl min-h-screen">
            <header class="flex gap-3 justify-between items-start">
                <div>
                    <h1 class="text-xl font-semibold tracking-tight">"WWM Patch Guard"</h1>
                    <p class="text-sm text-muted-foreground">
                        "Menjaga patch Bahasa Indonesia tetap terpasang."
                    </p>
                </div>
                <Summary ctx />
            </header>

            {move || {
                ctx.error
                    .get()
                    .map(|e| {
                        view! {
                            <Alert class="border-destructive/50 text-destructive">
                                <AlertTitle>"Tidak berhasil"</AlertTitle>
                                <AlertDescription>{e}</AlertDescription>
                            </Alert>
                        }
                    })
            }}

            <div class="flex flex-wrap gap-2">
                <Button on:click=add>"Tambah folder game"</Button>
                <Button
                    variant=ButtonVariant::Outline
                    attr:disabled=move || ctx.ui.with(|u| u.checking || u.dirs.is_empty())
                    on:click=move |_| ctx.run(api::check_now())
                >
                    {move || if ctx.ui.with(|u| u.checking) { "Memeriksa…" } else { "Periksa sekarang" }}
                </Button>
            </div>

            {move || {
                let dirs = ctx.ui.with(|u| u.dirs.clone());
                if dirs.is_empty() {
                    view! {
                        <Card>
                            <CardHeader>
                                <CardTitle>"Belum ada folder yang dijaga"</CardTitle>
                                <CardDescription>
                                    "Klik “Tambah folder game”, lalu pilih folder instalasi Where Winds Meet (folder launcher atau folder game-nya). Folder bahasa di dalamnya ditemukan otomatis."
                                </CardDescription>
                            </CardHeader>
                        </Card>
                    }
                        .into_any()
                } else {
                    dirs.into_iter().map(|d| view! { <DirCard ctx dir=d /> }).collect_view().into_any()
                }
            }}

            <Settings ctx />
            <Log ctx />

            <footer class="pt-1 text-xs text-center text-muted-foreground">
                {move || format!("v{}", ctx.ui.with(|u| u.version.clone()))}
                " · tutup jendela ini untuk menyembunyikan ke tray"
            </footer>
        </main>
    }
}

#[component]
fn Summary(ctx: Ctx) -> impl IntoView {
    move || {
        let (variant, text) = ctx.ui.with(|u| {
            let files = || u.dirs.iter().flat_map(|d| &d.files);
            if u.dirs.is_empty() {
                (BadgeVariant::Muted, "Belum diatur")
            } else if u.paused {
                (BadgeVariant::Warning, "Dijeda")
            } else if files().any(|f| f.status == Status::Error) {
                (BadgeVariant::Destructive, "Ada masalah")
            } else if files().any(|f| f.status == Status::Busy) {
                (BadgeVariant::Info, "Menunggu file")
            } else if files().any(|f| f.status == Status::Unknown) {
                (BadgeVariant::Warning, "Menunggu rilis")
            } else {
                (BadgeVariant::Success, "Terjaga")
            }
        });
        view! { <Badge variant=variant class="mt-1 shrink-0">{text}</Badge> }
    }
}

fn profile_title(p: Profile) -> &'static str {
    match p {
        Profile::Global => "Global",
        Profile::Cn => "China (燕云十六声)",
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
            <CardHeader>
                <CardTitle>{profile_title(dir.profile)}</CardTitle>
                <CardDescription class="text-xs break-all select-text">{dir.path.clone()}</CardDescription>
            </CardHeader>
            <CardContent class="flex flex-col gap-3">
                {dir.files.into_iter().map(|f| view! { <FileRow file=f /> }).collect_view()}
                <div class="flex flex-wrap gap-2 pt-1">
                    {has_unknown
                        .then(|| {
                            view! {
                                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm on:click=force>
                                    "Pasang paksa"
                                </Button>
                            }
                        })}
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=remove>
                        "Berhenti menjaga folder ini"
                    </Button>
                </div>
            </CardContent>
        </Card>
    }
}

#[component]
fn FileRow(file: FileReport) -> impl IntoView {
    let (variant, label) = status_badge(file.status);
    view! {
        <div class="flex gap-3 justify-between items-start">
            <div class="min-w-0">
                <p class="font-mono text-sm break-all">{file.name}</p>
                {(!file.detail.is_empty())
                    .then(|| view! { <p class="text-xs text-muted-foreground">{file.detail}</p> })}
            </div>
            <Badge variant=variant class="shrink-0">{label}</Badge>
        </div>
    }
}

#[component]
fn Settings(ctx: Ctx) -> impl IntoView {
    let restore = move |_| {
        if confirm(
            "Kembalikan file asli game di semua folder?\n\n\
             Penjaga akan dijeda supaya patch tidak dipasang lagi.",
        ) {
            ctx.run(api::restore_originals());
        }
    };
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Pengaturan"</CardTitle>
            </CardHeader>
            <CardContent class="flex flex-col gap-4">
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
                <div>
                    <Button
                        variant=ButtonVariant::Outline
                        size=ButtonSize::Sm
                        attr:disabled=move || ctx.ui.with(|u| u.dirs.is_empty())
                        on:click=restore
                    >
                        "Kembalikan file asli"
                    </Button>
                </div>
            </CardContent>
        </Card>
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
        <div class="flex gap-4 justify-between items-center">
            <div>
                <p class="text-sm font-medium">{title}</p>
                <p class="text-xs text-muted-foreground">{hint}</p>
            </div>
            <Switch checked=checked on_change=on_change aria_label=title />
        </div>
    }
}

fn clock(at: u64) -> String {
    let d = js_sys::Date::new(&JsValue::from_f64(at as f64 * 1000.0));
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}

#[component]
fn Log(ctx: Ctx) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Riwayat"</CardTitle>
                <CardDescription>
                    {move || match ctx.ui.with(|u| u.last_check) {
                        Some(at) => format!("Terakhir diperiksa {}", clock(at)),
                        None => "Belum diperiksa".to_string(),
                    }}
                </CardDescription>
            </CardHeader>
            <CardContent class="flex flex-col gap-1.5">
                {move || {
                    let log = ctx.ui.with(|u| u.log.clone());
                    if log.is_empty() {
                        view! { <p class="text-sm text-muted-foreground">"Belum ada kejadian."</p> }
                            .into_any()
                    } else {
                        log.into_iter()
                            .rev()
                            .take(12)
                            .map(|l| {
                                view! {
                                    <p class="text-xs select-text">
                                        <span class="font-mono text-muted-foreground">{clock(l.at)}</span>
                                        " "
                                        {l.text}
                                    </p>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }
                }}
            </CardContent>
        </Card>
    }
}
