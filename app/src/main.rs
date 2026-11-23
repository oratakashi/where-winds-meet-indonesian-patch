// No console window in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Started by an earlier Patch Guard to install an update: do that and nothing else.
    if let Some(code) = wwm_patch_guard::update::helper_main() {
        std::process::exit(code);
    }
    wwm_patch_guard::app::run()
}
