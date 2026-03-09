mod commands;
mod crypto;
mod db;
mod error;

use std::sync::Mutex;
use tauri::Manager;

use commands::auth::VaultState;
use commands::totp::TotpState;

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
            app.manage(TotpState {
                enabled: Mutex::new(false),
                secret: Mutex::new(None),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::vault_exists,
            commands::auth::is_locked,
            commands::auth::setup_vault,
            commands::auth::unlock,
            commands::auth::lock,
            commands::totp::totp_generate_secret,
            commands::totp::totp_get_qr_base64,
            commands::totp::totp_get_url,
            commands::totp::totp_verify,
            commands::totp::totp_enable,
            commands::totp::totp_is_enabled,
            commands::expenses::get_expenses,
            commands::expenses::create_expense,
            commands::expenses::update_expense,
            commands::expenses::delete_expense,
            commands::subscriptions::get_subscriptions,
            commands::subscriptions::create_subscription,
            commands::subscriptions::update_subscription,
            commands::subscriptions::delete_subscription,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
