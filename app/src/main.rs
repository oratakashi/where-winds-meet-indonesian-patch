// No console window in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wwm_patch_guard::shell::system;

fn main() {
    system::harden_dll_search();
    // Started by an earlier Patch Guard to install an update: do that and nothing else.
    if let Some(code) = wwm_patch_guard::update::helper_main(system::data_dir(), system::installed_exe) {
        std::process::exit(code);
    }
    wwm_patch_guard::app::run()
}
