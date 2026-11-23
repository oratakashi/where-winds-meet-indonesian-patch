//! What the window shows, and the actions behind its buttons — the Tauri commands of v0.3,
//! now plain method calls on the engine.
use crate::app::Cmd;
use crate::engine::{Engine, Msg, Snapshot};
use crate::locations;
use crate::model::{Client, Profile};
use crate::shell::autostart;
use gpui::{App, Context, PathPromptOptions, SharedString};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Settings,
    History,
}

/// A locale folder of a game found on this machine.
#[derive(Clone)]
pub struct Found {
    pub profile: Profile,
    pub client: Client,
    pub path: PathBuf,
    pub guarded: bool,
}

/// A question shown as a `ContentDialog`; `action` runs on the primary button.
#[derive(Clone)]
pub struct Confirm {
    pub title: SharedString,
    pub body: SharedString,
    pub primary: SharedString,
    pub action: Rc<dyn Fn(&mut AppState, &mut Context<AppState>)>,
}

pub struct AppState {
    engine: Arc<Engine>,
    cmd: async_channel::Sender<Cmd>,
    pub snap: Snapshot,
    pub autostart: bool,
    /// Game folders found on this machine (guarded or not).
    pub found: Vec<Found>,
    pub error: Option<SharedString>,
    pub page: Page,
    pub confirm: Option<Confirm>,
}

impl AppState {
    pub fn new(engine: Arc<Engine>, cmd: async_channel::Sender<Cmd>, cx: &mut Context<Self>) -> Self {
        let mut s = AppState {
            snap: engine.snapshot(),
            engine,
            cmd,
            autostart: autostart::is_enabled(),
            found: Vec::new(),
            error: None,
            page: Page::Home,
            confirm: None,
        };
        s.rescan(cx);
        s
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Reload the engine's state (after every pass, and after every action).
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        self.snap = self.engine.snapshot();
        cx.notify();
    }

    /// Look for installed copies of the game again (Steam libraries, NetEase launchers).
    pub fn rescan(&mut self, cx: &mut Context<Self>) {
        let dirs = self.engine.config().dirs;
        cx.spawn(async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move {
                    locations::discover()
                        .into_iter()
                        .map(|g| Found {
                            profile: g.profile,
                            client: Client::of(&g.path),
                            guarded: dirs.iter().any(|d| locations::same_path(&d.path, &g.path)),
                            path: g.path,
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            let _ = this.update(cx, |s, cx| {
                s.found = found;
                cx.notify();
            });
        })
        .detach();
    }

    /// Show the outcome of an action, then reload everything.
    fn done(&mut self, r: Result<(), String>, cx: &mut Context<Self>) {
        self.error = r.err().map(Into::into);
        self.autostart = autostart::is_enabled();
        self.refresh(cx);
        self.rescan(cx);
    }

    pub fn show_error(&mut self, e: String, cx: &mut Context<Self>) {
        self.error = Some(e.into());
        cx.notify();
    }

    pub fn dismiss_error(&mut self, cx: &mut Context<Self>) {
        self.error = None;
        cx.notify();
    }

    pub fn go(&mut self, page: Page, cx: &mut Context<Self>) {
        self.page = page;
        cx.notify();
    }

    pub fn ask(&mut self, confirm: Confirm, cx: &mut Context<Self>) {
        self.confirm = Some(confirm);
        cx.notify();
    }

    pub fn answer(&mut self, yes: bool, cx: &mut Context<Self>) {
        if let Some(c) = self.confirm.take() {
            if yes {
                (c.action)(self, cx);
            }
        }
        cx.notify();
    }

    pub fn check_now(&mut self, cx: &mut Context<Self>) {
        self.engine.send(Msg::Check { force_remote: true });
        self.done(Ok(()), cx);
    }

    pub fn set_paused(&mut self, paused: bool, cx: &mut Context<Self>) {
        let r = self.engine.update_config(|c| c.paused = paused);
        self.done(r, cx);
    }

    pub fn set_auto_update(&mut self, enabled: bool, cx: &mut Context<Self>) {
        let r = self.engine.update_config(|c| c.auto_update = enabled);
        self.done(r, cx);
    }

    pub fn set_auto_update_app(&mut self, enabled: bool, cx: &mut Context<Self>) {
        let r = self.engine.update_config(|c| c.auto_update_app = enabled);
        self.done(r, cx);
    }

    pub fn check_app_update(&mut self, cx: &mut Context<Self>) {
        self.engine.check_app_update();
        self.snap.app_update_checking = true;
        cx.notify();
    }

    /// Install the downloaded Patch Guard update; the app closes and starts again.
    pub fn restart_for_update(&mut self, _cx: &mut Context<Self>) {
        let _ = self.cmd.try_send(Cmd::RestartForUpdate);
    }

    pub fn set_autostart(&mut self, enabled: bool, cx: &mut Context<Self>) {
        let r = autostart::set(enabled);
        self.done(r, cx);
    }

    /// Put the original files back and pause the guard, so it does not patch them again.
    pub fn restore_originals(&mut self, cx: &mut Context<Self>) {
        let r = self.engine.update_config(|c| c.paused = true);
        if r.is_ok() {
            self.engine.send(Msg::Uninstall);
        }
        self.done(r, cx);
    }

    /// Install the latest release into a folder even where the installed file is unknown.
    pub fn force_install(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let r = if self.engine.config().dirs.iter().any(|d| d.path == path) {
            self.engine.send(Msg::ForceInstall(path));
            Ok(())
        } else {
            Err("folder itu tidak dijaga".to_string())
        };
        self.done(r, cx);
    }

    pub fn remove_folder(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let r = self.engine.update_config(|c| c.dirs.retain(|d| d.path != path));
        self.done(r, cx);
    }

    pub fn clear_history(&mut self, cx: &mut Context<Self>) {
        self.engine.clear_log();
        self.done(Ok(()), cx);
    }

    /// Guard detected folders. Only folders `locations::discover` reports are accepted.
    pub fn add_detected(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        let found = locations::discover();
        let r = self.engine.update_config(|c| {
            for g in found {
                let known = c.dirs.iter().any(|d| locations::same_path(&d.path, &g.path));
                if paths.contains(&g.path) && !known {
                    c.dirs.push(g);
                }
            }
        });
        self.done(r, cx);
    }

    /// Ask for a folder and guard every locale folder found from it.
    pub fn add_folder(&mut self, cx: &mut Context<Self>) {
        let picked = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Pilih folder".into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = picked.await else { return };
            let Some(picked) = paths.into_iter().next() else { return };
            let found = cx
                .background_executor()
                .spawn(async move { locations::detect(&picked) })
                .await;
            let _ = this.update(cx, |s, cx| {
                let r = if found.is_empty() {
                    Err("Tidak ada file bahasa game (translate_words_map_*) di folder itu. \
                         Pilih folder instalasi game atau launcher-nya."
                        .to_string())
                } else {
                    let mut added = 0;
                    s.engine
                        .update_config(|c| {
                            for g in found {
                                if !c.dirs.contains(&g) {
                                    c.dirs.push(g);
                                    added += 1;
                                }
                            }
                        })
                        .and_then(|()| {
                            if added == 0 { Err("Folder itu sudah dijaga.".to_string()) } else { Ok(()) }
                        })
                };
                s.done(r, cx);
            });
        })
        .detach();
    }
}

/// Used by `app.rs` outside an entity context.
pub fn refresh(state: &gpui::Entity<AppState>, cx: &mut App) {
    state.update(cx, |s, cx| s.refresh(cx));
}
