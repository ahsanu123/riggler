use crate::commands::{get_config, set_config, toggle_jiggling};

mod commands;
mod riggler_config;
mod riggler_err;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    riggler_shared::jiggling();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_config,
            toggle_jiggling
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
