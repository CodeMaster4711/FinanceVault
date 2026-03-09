use std::sync::Mutex;
use tauri::State;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::commands::auth::VaultState;
use crate::error::{Result, VaultError};

pub struct TotpState {
    pub enabled: Mutex<bool>,
    pub secret: Mutex<Option<String>>,
}

fn build_totp(secret_base32: &str, account: &str) -> Result<TOTP> {
    let bytes = Secret::Encoded(secret_base32.to_string())
        .to_bytes()
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        bytes,
        Some("FinanceVault".to_string()),
        account.to_string(),
    )
    .map_err(|e| VaultError::Crypto(e.to_string()))
}

fn new_secret_base32() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 20];
    rand::thread_rng().fill_bytes(&mut bytes);
    Secret::Raw(bytes.to_vec()).to_encoded().to_string()
}

#[tauri::command]
pub fn totp_generate_secret() -> String {
    new_secret_base32()
}

#[tauri::command]
pub fn totp_get_qr_base64(secret_base32: String, account: String) -> Result<String> {
    let totp = build_totp(&secret_base32, &account)?;
    totp.get_qr_base64().map_err(|e| VaultError::Crypto(e))
}

#[tauri::command]
pub fn totp_get_url(secret_base32: String, account: String) -> Result<String> {
    let totp = build_totp(&secret_base32, &account)?;
    Ok(totp.get_url())
}

#[tauri::command]
pub fn totp_verify(code: String, secret_base32: String) -> Result<bool> {
    let totp = build_totp(&secret_base32, "verify")?;
    Ok(totp.check_current(&code).unwrap_or(false))
}

#[tauri::command]
pub fn totp_enable(
    secret_base32: String,
    code: String,
    state: State<'_, VaultState>,
    totp_state: State<'_, TotpState>,
) -> Result<()> {
    let totp = build_totp(&secret_base32, "FinanceVault")?;
    if !totp.check_current(&code).unwrap_or(false) {
        return Err(VaultError::InvalidTotp);
    }

    let key_guard = state.key.lock().unwrap();
    let key = key_guard.as_ref().ok_or(VaultError::Locked)?;

    let encrypted = crate::crypto::encrypt(key, secret_base32.as_bytes())?;

    let conn = crate::db::open(&state.db_path)?;
    conn.execute(
        "INSERT INTO vault_meta (key, value) VALUES ('totp_secret', ?1)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [&encrypted],
    )?;
    conn.execute(
        "INSERT INTO vault_meta (key, value) VALUES ('totp_enabled', '1')
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [],
    )?;

    *totp_state.enabled.lock().unwrap() = true;
    *totp_state.secret.lock().unwrap() = Some(secret_base32);
    Ok(())
}

#[tauri::command]
pub fn totp_is_enabled(totp_state: State<'_, TotpState>) -> bool {
    *totp_state.enabled.lock().unwrap()
}

pub fn load_totp_state(
    conn: &rusqlite::Connection,
    key: &crate::crypto::DerivedKey,
) -> Result<(bool, Option<String>)> {
    let enabled = conn
        .query_row(
            "SELECT value FROM vault_meta WHERE key = 'totp_enabled'",
            [],
            |row| row.get::<_, String>(0),
        )
        .map(|v| v == "1")
        .unwrap_or(false);

    if !enabled {
        return Ok((false, None));
    }

    let encrypted = conn
        .query_row(
            "SELECT value FROM vault_meta WHERE key = 'totp_secret'",
            [],
            |row| row.get::<_, String>(0),
        )
        .map_err(VaultError::Database)?;

    let plain = crate::crypto::decrypt(key, &encrypted)?;
    let secret = String::from_utf8(plain).map_err(|e| VaultError::Crypto(e.to_string()))?;

    Ok((true, Some(secret)))
}
