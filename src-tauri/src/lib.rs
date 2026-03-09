mod crypto;
mod db;
mod commands;
mod error;

use std::sync::Mutex;
use tauri::Manager;
use commands::auth::VaultState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");
            std::fs::create_dir_all(&data_dir)?;
            let db_path = data_dir.join("vault.db");

            app.manage(VaultState {
                key: Mutex::new(None),
                db_path,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::unlock,
            commands::auth::lock,
            commands::auth::is_locked,
            commands::auth::setup_vault,
            commands::auth::vault_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
