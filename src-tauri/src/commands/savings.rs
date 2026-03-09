use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::commands::auth::VaultState;
use crate::db;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavingsPlan {
    pub id: String,
    pub name: String,
    pub isin: String,
    pub ticker: String,
    pub amount: f64,
    pub currency: String,
    pub interval: String,
    pub next_date: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSavingsPlan {
    pub name: String,
    pub isin: String,
    pub ticker: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub interval: String,
    pub next_date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSavingsPlan {
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub interval: Option<String>,
    pub next_date: Option<String>,
}

fn require_unlocked(state: &VaultState) -> Result<()> {
    if state.key.lock().unwrap().is_none() {
        return Err(VaultError::Locked);
    }
    Ok(())
}

#[tauri::command]
pub fn get_savings_plans(state: State<'_, VaultState>) -> Result<Vec<SavingsPlan>> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let mut stmt = conn
        .prepare(
            "SELECT id, name, isin, ticker, amount, currency, interval, next_date, created_at
             FROM savings_plans ORDER BY next_date ASC",
        )
        .map_err(VaultError::Database)?;

    let rows = stmt
        .query_map([], |row| {
            Ok(SavingsPlan {
                id: row.get(0)?,
                name: row.get(1)?,
                isin: row.get(2)?,
                ticker: row.get(3)?,
                amount: row.get(4)?,
                currency: row.get(5)?,
                interval: row.get(6)?,
                next_date: row.get(7)?,
                created_at: row.get(8)?,
            })
        })
        .map_err(VaultError::Database)?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(VaultError::Database)
}

#[tauri::command]
pub fn create_savings_plan(input: CreateSavingsPlan, state: State<'_, VaultState>) -> Result<SavingsPlan> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let currency = input.currency.unwrap_or_else(|| "EUR".to_string());

    conn.execute(
        "INSERT INTO savings_plans (id, name, isin, ticker, amount, currency, interval, next_date, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![id, input.name, input.isin, input.ticker, input.amount, currency, input.interval, input.next_date, now],
    )
    .map_err(VaultError::Database)?;

    Ok(SavingsPlan {
        id,
        name: input.name,
        isin: input.isin,
        ticker: input.ticker,
        amount: input.amount,
        currency,
        interval: input.interval,
        next_date: input.next_date,
        created_at: now,
    })
}

#[tauri::command]
pub fn update_savings_plan(id: String, input: UpdateSavingsPlan, state: State<'_, VaultState>) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;

    if let Some(name) = input.name {
        conn.execute("UPDATE savings_plans SET name = ?1 WHERE id = ?2", [&name, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(amount) = input.amount {
        conn.execute("UPDATE savings_plans SET amount = ?1 WHERE id = ?2", rusqlite::params![amount, id])
            .map_err(VaultError::Database)?;
    }
    if let Some(currency) = input.currency {
        conn.execute("UPDATE savings_plans SET currency = ?1 WHERE id = ?2", [&currency, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(interval) = input.interval {
        conn.execute("UPDATE savings_plans SET interval = ?1 WHERE id = ?2", [&interval, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(next_date) = input.next_date {
        conn.execute("UPDATE savings_plans SET next_date = ?1 WHERE id = ?2", [&next_date, &id])
            .map_err(VaultError::Database)?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_savings_plan(id: String, state: State<'_, VaultState>) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    conn.execute("DELETE FROM savings_plans WHERE id = ?1", [&id])
        .map_err(VaultError::Database)?;
    Ok(())
}
