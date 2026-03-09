use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

use crate::crypto::{derive_key, generate_salt_b64, encrypt, decrypt, DerivedKey};
use crate::db;
use crate::error::{Result, VaultError};

pub struct VaultState {
    pub key: Mutex<Option<DerivedKey>>,
    pub db_path: PathBuf,
}

fn meta_get(conn: &rusqlite::Connection, key: &str) -> rusqlite::Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM vault_meta WHERE key = ?1")?;
    let mut rows = stmt.query([key])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
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
pub fn unlock(passphrase: String, state: State<'_, VaultState>) -> Result<()> {
    if !state.db_path.exists() {
        return Err(VaultError::NotInitialized);
    }

    let conn = db::open(&state.db_path)?;

    let salt = meta_get(&conn, "salt")?
        .ok_or(VaultError::NotInitialized)?;
    let canary = meta_get(&conn, "canary")?
        .ok_or(VaultError::NotInitialized)?;

    let key = derive_key(&passphrase, &salt)?;
    let plaintext = decrypt(&key, &canary)?;

    if plaintext != b"financevault" {
        return Err(VaultError::InvalidPassphrase);
    }

    *state.key.lock().unwrap() = Some(key);
    Ok(())
}

#[tauri::command]
pub fn lock(state: State<'_, VaultState>) -> Result<()> {
    *state.key.lock().unwrap() = None;
    Ok(())
}
