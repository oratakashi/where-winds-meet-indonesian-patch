//! The window shell: navigation pane on the left (WinUI `NavigationView` style), the selected
//! page on the right. The pages live in `pages/`.
use crate::api::{self, Client, Found, Profile, Ui};
use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};
use crate::pages::history::History;
use crate::pages::home::Home;
use crate::pages::settings::Settings;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::future::Future;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn confirm(message: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function set_mica(on) { document.documentElement.classList.toggle('mica', on); }")]
extern "C" {
    fn set_mica(on: bool);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Settings,
    History,
}

/// Shared by every part of the window.
#[derive(Clone, Copy)]
pub struct Ctx {
    pub ui: RwSignal<Ui>,
    pub error: RwSignal<Option<String>>,
    /// Game folders found on this machine (guarded or not).
    pub found: RwSignal<Vec<Found>>,
    pub page: RwSignal<Page>,
}

impl Ctx {
    pub fn refresh(self) {
        spawn_local(async move {
            match api::get_status().await {
                Ok(u) => self.ui.set(u),
                Err(e) => self.error.set(Some(e)),
            }
        });
    }

    /// Look for installed copies of the game again.
    pub fn rescan(self) {
        spawn_local(async move {
            if let Ok(f) = api::detect_installs().await {
                self.found.set(f);
            }
        });
    }

    /// Run a backend command, show its error if any, then reload everything.
    pub fn run(self, fut: impl Future<Output = Result<(), String>> + 'static) {
        spawn_local(async move {
            match fut.await {
                Ok(()) => self.error.set(None),
                Err(e) => self.error.set(Some(e)),
            }
            self.refresh();
            self.rescan();
        });
    }
}

#[component]
pub fn App() -> impl IntoView {
    let ctx = Ctx {
        ui: RwSignal::new(Ui::default()),
        error: RwSignal::new(None),
        found: RwSignal::new(Vec::new()),
        page: RwSignal::new(Page::Home),
    };
    ctx.refresh();
    ctx.rescan();
    spawn_local(api::on_status(move || ctx.refresh()));
    Effect::new(move |_| set_mica(ctx.ui.with(|u| u.mica)));

    view! {
        <div class="flex h-screen">
            <nav class="flex flex-col gap-1 p-2 w-52 shrink-0">
                <p class="px-3 pt-3 pb-4 font-semibold">"WWM Patch Guard"</p>
                <NavItem ctx page=Page::Home glyph="\u{E80F}" label="Home" />
                <NavItem ctx page=Page::Settings glyph="\u{E713}" label="Settings" />
                <NavItem ctx page=Page::History glyph="\u{E81C}" label="History" />
                <p class="px-3 pb-2 mt-auto text-xs text-muted-foreground">
                    {move || format!("Versi {}", ctx.ui.with(|u| u.version.clone()))}
                </p>
            </nav>

            <main class="overflow-y-auto min-w-0 grow">
                <div class="flex flex-col gap-3 px-6 pt-5 pb-6 mx-auto max-w-3xl">
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
                    {move || match ctx.page.get() {
                        Page::Home => view! { <Home ctx /> }.into_any(),
                        Page::Settings => view! { <Settings ctx /> }.into_any(),
                        Page::History => view! { <History ctx /> }.into_any(),
                    }}
                </div>
            </main>
        </div>
    }
}

#[component]
fn NavItem(ctx: Ctx, page: Page, glyph: &'static str, label: &'static str) -> impl IntoView {
    let selected = move || ctx.page.get() == page;
    view! {
        <button
            class=move || {
                format!(
                    "relative flex items-center gap-3 h-9 px-3 rounded-[4px] text-left outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring/50 {}",
                    if selected() { "bg-accent" } else { "" },
                )
            }
            aria-current=move || selected().then_some("page")
            on:click=move |_| ctx.page.set(page)
        >
            {move || {
                selected()
                    .then(|| view! { <span class="absolute left-0 top-2.5 h-4 rounded-full w-[3px] bg-primary"></span> })
            }}
            <span class="text-base icon">{glyph}</span>
            {label}
        </button>
    }
}

#[component]
pub fn PageTitle(text: &'static str) -> impl IntoView {
    view! { <h1 class="mb-1 font-semibold leading-9 text-[28px]">{text}</h1> }
}

pub fn profile_title(p: Profile) -> &'static str {
    match p {
        Profile::Global => "Global",
        Profile::Cn => "China (燕云十六声)",
    }
}

pub fn client_title(c: Client) -> &'static str {
    match c {
        Client::Steam => "Steam",
        Client::Launcher => "Launcher resmi",
    }
}

/// `HH:MM` of a Unix timestamp, local time.
pub fn clock(at: u64) -> String {
    let d = js_sys::Date::new(&JsValue::from_f64(at as f64 * 1000.0));
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}

/// `13 Nov 2026` of a Unix timestamp, local time.
pub fn day(at: u64) -> String {
    const MONTHS: [&str; 12] =
        ["Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des"];
    let d = js_sys::Date::new(&JsValue::from_f64(at as f64 * 1000.0));
    format!("{} {} {}", d.get_date(), MONTHS[d.get_month() as usize % 12], d.get_full_year())
}
