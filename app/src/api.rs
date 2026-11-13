//! Bridge to the Tauri backend (`src-tauri/src/lib.rs`): commands and the `status` event.
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn tauri_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen, catch)]
    async fn tauri_listen(event: &str, handler: &js_sys::Function) -> Result<JsValue, JsValue>;
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    Global,
    Cn,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Client {
    Steam,
    Launcher,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Patched,
    Original,
    Unknown,
    Missing,
    Busy,
    Error,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct FileReport {
    pub name: String,
    pub status: Status,
    pub detail: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct DirReport {
    pub profile: Profile,
    pub client: Client,
    pub path: String,
    pub files: Vec<FileReport>,
}

/// A locale folder of a game found on this machine.
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Found {
    pub profile: Profile,
    pub client: Client,
    pub path: String,
    pub guarded: bool,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct LogLine {
    pub at: u64,
    pub text: String,
}

#[derive(Deserialize, Clone, PartialEq, Default, Debug)]
pub struct Ui {
    pub dirs: Vec<DirReport>,
    pub paused: bool,
    pub auto_update: bool,
    pub checking: bool,
    pub last_check: Option<u64>,
    pub log: Vec<LogLine>,
    pub autostart: bool,
    pub version: String,
    pub mica: bool,
}

#[derive(Serialize)]
struct NoArgs {}

#[derive(Serialize)]
struct PathArg<'a> {
    path: &'a str,
}

#[derive(Serialize)]
struct PathsArg<'a> {
    paths: &'a [String],
}

#[derive(Serialize)]
struct Flag {
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

async fn call<T: DeserializeOwned>(cmd: &str, args: impl Serialize) -> Result<T, String> {
    let args = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let out = tauri_invoke(cmd, args)
        .await
        .map_err(|e| e.as_string().unwrap_or_else(|| "perintah gagal".to_string()))?;
    serde_wasm_bindgen::from_value(out).map_err(|e| e.to_string())
}

pub async fn get_status() -> Result<Ui, String> {
    call("get_status", NoArgs {}).await
}

/// `Ok(None)`: the folder dialog was cancelled.
pub async fn add_folder() -> Result<Option<usize>, String> {
    call("add_folder", NoArgs {}).await
}

pub async fn detect_installs() -> Result<Vec<Found>, String> {
    call("detect_installs", NoArgs {}).await
}

/// Guard folders reported by `detect_installs`; returns how many were added.
pub async fn add_detected(paths: &[String]) -> Result<usize, String> {
    call("add_detected", PathsArg { paths }).await
}

pub async fn clear_history() -> Result<(), String> {
    call("clear_history", NoArgs {}).await
}

pub async fn remove_folder(path: &str) -> Result<(), String> {
    call("remove_folder", PathArg { path }).await
}

pub async fn force_install(path: &str) -> Result<(), String> {
    call("force_install", PathArg { path }).await
}

pub async fn check_now() -> Result<(), String> {
    call("check_now", NoArgs {}).await
}

pub async fn restore_originals() -> Result<(), String> {
    call("restore_originals", NoArgs {}).await
}

pub async fn set_paused(paused: bool) -> Result<(), String> {
    call("set_paused", Flag { paused: Some(paused), enabled: None }).await
}

pub async fn set_auto_update(enabled: bool) -> Result<(), String> {
    call("set_auto_update", Flag { paused: None, enabled: Some(enabled) }).await
}

pub async fn set_autostart(enabled: bool) -> Result<(), String> {
    call("set_autostart", Flag { paused: None, enabled: Some(enabled) }).await
}

/// Run `f` every time the backend finishes a pass. The listener lives as long as the page.
pub async fn on_status(f: impl Fn() + 'static) {
    let cb = Closure::<dyn Fn(JsValue)>::new(move |_| f());
    let _ = tauri_listen("status", cb.as_ref().unchecked_ref()).await;
    cb.forget();
}
