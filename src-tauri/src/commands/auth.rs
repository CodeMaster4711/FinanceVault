use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::commands::totp::{load_totp_state, TotpState};
use crate::crypto::{decrypt, derive_key, encrypt, generate_salt_b64, DerivedKey};
use crate::db;
use crate::error::{Result, VaultError};

pub struct VaultState {
    pub key: Mutex<Option<DerivedKey>>,
    pub db_path: PathBuf,
}

fn meta_get(conn: &rusqlite::Connection, key: &str) -> rusqlite::Result<Option<String>> {
    conn.query_row(
        "SELECT value FROM vault_meta WHERE key = ?1",
        [key],
        |row| row.get::<_, String>(0),
    )
    .map(Some)
    .or_else(|e| {
        if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
            Ok(None)
        } else {
            Err(e)
        }
    })
}

fn meta_set(conn: &rusqlite::Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO vault_meta (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [key, value],
    )?;
    Ok(())
}

#[tauri::command]
pub fn vault_exists(state: State<'_, VaultState>) -> bool {
    state.db_path.exists()
}

#[tauri::command]
pub fn is_locked(state: State<'_, VaultState>) -> bool {
    state.key.lock().unwrap().is_none()
}

#[tauri::command]
pub fn setup_vault(passphrase: String, state: State<'_, VaultState>) -> Result<()> {
    if state.db_path.exists() {
        return Err(VaultError::Crypto("vault already exists".into()));
    }

    let salt = generate_salt_b64();
    let key = derive_key(&passphrase, &salt)?;

    let conn = db::open(&state.db_path)?;
    db::migrate(&conn)?;
    meta_set(&conn, "salt", &salt)?;

    let canary = encrypt(&key, b"financevault")?;
    meta_set(&conn, "canary", &canary)?;

    *state.key.lock().unwrap() = Some(key);
    Ok(())
}

#[tauri::command]
pub fn unlock(
    passphrase: String,
    totp_code: Option<String>,
    state: State<'_, VaultState>,
    totp_state: State<'_, TotpState>,
) -> Result<()> {
    if !state.db_path.exists() {
        return Err(VaultError::NotInitialized);
    }

    let conn = db::open(&state.db_path)?;

    let salt = meta_get(&conn, "salt")?.ok_or(VaultError::NotInitialized)?;
    let canary = meta_get(&conn, "canary")?.ok_or(VaultError::NotInitialized)?;

    let key = derive_key(&passphrase, &salt)?;
    let plaintext = decrypt(&key, &canary)?;

    if plaintext != b"financevault" {
        return Err(VaultError::InvalidPassphrase);
    }

    let (totp_enabled, totp_secret) = load_totp_state(&conn, &key)?;
    if totp_enabled {
        let secret = totp_secret.ok_or(VaultError::InvalidTotp)?;
        let code = totp_code.ok_or(VaultError::InvalidTotp)?;

        let bytes = Secret::Encoded(secret.clone())
            .to_bytes()
            .map_err(|e| VaultError::Crypto(e.to_string()))?;
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            bytes,
            Some("FinanceVault".to_string()),
            "FinanceVault".to_string(),
        )
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

        if !totp.check_current(&code).unwrap_or(false) {
            return Err(VaultError::InvalidTotp);
        }

        *totp_state.enabled.lock().unwrap() = true;
        *totp_state.secret.lock().unwrap() = Some(secret);
    }

    *state.key.lock().unwrap() = Some(key);
    Ok(())
}

#[tauri::command]
pub fn lock(state: State<'_, VaultState>, totp_state: State<'_, TotpState>) -> Result<()> {
    *state.key.lock().unwrap() = None;
    *totp_state.secret.lock().unwrap() = None;
    *totp_state.enabled.lock().unwrap() = false;
    Ok(())
}
