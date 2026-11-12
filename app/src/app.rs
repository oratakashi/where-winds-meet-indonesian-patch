use crate::api::{self, DirReport, FileReport, Profile, Status, Ui};
use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardContent};
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

#[wasm_bindgen(inline_js = "export function set_mica(on) { document.documentElement.classList.toggle('mica', on); }")]
extern "C" {
    fn set_mica(on: bool);
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
    Effect::new(move |_| set_mica(ctx.ui.with(|u| u.mica)));

    let add = move |_| {
        ctx.run(async move {
            match api::add_folder().await? {
                Some(0) => Err("Folder itu sudah dijaga.".to_string()),
                _ => Ok(()),
            }
        })
    };

    view! {
        <main class="flex flex-col gap-3 px-6 pt-5 pb-6 mx-auto max-w-2xl min-h-screen">
            <h1 class="mb-1 text-[28px] leading-9 font-semibold">"WWM Patch Guard"</h1>

            <Hero ctx />

            {move || {
                ctx.error
                    .get()
                    .map(|e| {
                        view! {
                            <Alert class="border-destructive/40 bg-destructive/10">
                                <AlertTitle>"Tidak berhasil"</AlertTitle>
                                <AlertDescription>{e}</AlertDescription>
                            </Alert>
                        }
                    })
            }}

            <div class="flex gap-3 justify-between items-center mt-3">
                <h2 class="text-sm font-semibold">"Folder yang dijaga"</h2>
                <Button size=ButtonSize::Sm on:click=add>
                    <span class="icon">"\u{E710}"</span>
                    "Tambah folder"
                </Button>
            </div>

            {move || {
                let dirs = ctx.ui.with(|u| u.dirs.clone());
                if dirs.is_empty() {
                    view! {
                        <Card>
                            <CardContent class="flex flex-col gap-1">
                                <p class="font-medium">"Belum ada folder yang dijaga"</p>
                                <p class="text-muted-foreground">
                                    "Pilih folder instalasi Where Winds Meet (folder launcher atau folder game-nya). Folder bahasa di dalamnya ditemukan otomatis."
                                </p>
                            </CardContent>
                        </Card>
                    }
                        .into_any()
                } else {
                    dirs.into_iter().map(|d| view! { <DirCard ctx dir=d /> }).collect_view().into_any()
                }
            }}

            <h2 class="mt-3 text-sm font-semibold">"Pengaturan"</h2>
            <Settings ctx />

            <h2 class="mt-3 text-sm font-semibold">"Riwayat"</h2>
            <Log ctx />

            <footer class="pt-3 text-xs text-center text-muted-foreground">
                {move || format!("Versi {}", ctx.ui.with(|u| u.version.clone()))}
                " · menutup jendela ini menyembunyikannya ke tray"
            </footer>
        </main>
    }
}

/// Status summary, like the banner at the top of Windows Update.
#[component]
fn Hero(ctx: Ctx) -> impl IntoView {
    move || {
        let (glyph, tone, title, sub) = ctx.ui.with(|u| {
            let files = || u.dirs.iter().flat_map(|d| &d.files);
            if u.dirs.is_empty() {
                ("\u{E710}", "text-muted-foreground", "Belum diatur", "Tambahkan folder game untuk mulai menjaga patch.")
            } else if u.paused {
                ("\u{E769}", "text-warning", "Penjaga dijeda", "File game tidak diubah sampai dilanjutkan.")
            } else if files().any(|f| f.status == Status::Error) {
                ("\u{EA39}", "text-destructive", "Ada masalah", "Lihat riwayat di bawah untuk detailnya.")
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
            <CardContent class="flex flex-col gap-0.5">
                <p class="font-semibold">{profile_title(dir.profile)}</p>
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
        <div class="flex gap-3 justify-between items-center px-4 py-2.5">
            <div class="min-w-0">
                <p class="font-mono text-[13px] break-all">{file.name}</p>
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
        <Card class="py-0 gap-0">
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

fn clock(at: u64) -> String {
    let d = js_sys::Date::new(&JsValue::from_f64(at as f64 * 1000.0));
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}

#[component]
fn Log(ctx: Ctx) -> impl IntoView {
    view! {
        <Card>
            <CardContent class="flex flex-col gap-1.5">
                <p class="text-xs text-muted-foreground">
                    {move || match ctx.ui.with(|u| u.last_check) {
                        Some(at) => format!("Terakhir diperiksa {}", clock(at)),
                        None => "Belum diperiksa".to_string(),
                    }}
                </p>
                {move || {
                    let log = ctx.ui.with(|u| u.log.clone());
                    if log.is_empty() {
                        view! { <p class="text-muted-foreground">"Belum ada kejadian."</p> }.into_any()
                    } else {
                        log.into_iter()
                            .rev()
                            .take(12)
                            .map(|l| {
                                view! {
                                    <p class="text-[13px] select-text">
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
