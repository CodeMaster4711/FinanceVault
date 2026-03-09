use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::commands::auth::VaultState;
use crate::db;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub currency: String,
    pub billing: String,
    pub next_billing: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscription {
    pub name: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub billing: String,
    pub next_billing: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubscription {
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub billing: Option<String>,
    pub next_billing: Option<String>,
}

fn require_unlocked(state: &VaultState) -> Result<()> {
    if state.key.lock().unwrap().is_none() {
        return Err(VaultError::Locked);
    }
    Ok(())
}

#[tauri::command]
pub fn get_subscriptions(state: State<'_, VaultState>) -> Result<Vec<Subscription>> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let mut stmt = conn
        .prepare(
            "SELECT id, name, amount, currency, billing, next_billing, created_at
             FROM subscriptions ORDER BY next_billing ASC",
        )
        .map_err(VaultError::Database)?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Subscription {
                id: row.get(0)?,
                name: row.get(1)?,
                amount: row.get(2)?,
                currency: row.get(3)?,
                billing: row.get(4)?,
                next_billing: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(VaultError::Database)?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(VaultError::Database)
}

#[tauri::command]
pub fn create_subscription(
    input: CreateSubscription,
    state: State<'_, VaultState>,
) -> Result<Subscription> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let currency = input.currency.unwrap_or_else(|| "EUR".to_string());

    conn.execute(
        "INSERT INTO subscriptions (id, name, amount, currency, billing, next_billing, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![id, input.name, input.amount, currency, input.billing, input.next_billing, now],
    )
    .map_err(VaultError::Database)?;

    Ok(Subscription {
        id,
        name: input.name,
        amount: input.amount,
        currency,
        billing: input.billing,
        next_billing: input.next_billing,
        created_at: now,
    })
}

#[tauri::command]
pub fn update_subscription(
    id: String,
    input: UpdateSubscription,
    state: State<'_, VaultState>,
) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;

    if let Some(name) = input.name {
        conn.execute("UPDATE subscriptions SET name = ?1 WHERE id = ?2", [&name, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(amount) = input.amount {
        conn.execute(
            "UPDATE subscriptions SET amount = ?1 WHERE id = ?2",
            rusqlite::params![amount, id],
        )
        .map_err(VaultError::Database)?;
    }
    if let Some(currency) = input.currency {
        conn.execute(
            "UPDATE subscriptions SET currency = ?1 WHERE id = ?2",
            [&currency, &id],
        )
        .map_err(VaultError::Database)?;
    }
    if let Some(billing) = input.billing {
        conn.execute(
            "UPDATE subscriptions SET billing = ?1 WHERE id = ?2",
            [&billing, &id],
        )
        .map_err(VaultError::Database)?;
    }
    if let Some(next_billing) = input.next_billing {
        conn.execute(
            "UPDATE subscriptions SET next_billing = ?1 WHERE id = ?2",
            [&next_billing, &id],
        )
        .map_err(VaultError::Database)?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_subscription(id: String, state: State<'_, VaultState>) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    conn.execute("DELETE FROM subscriptions WHERE id = ?1", [&id])
        .map_err(VaultError::Database)?;
    Ok(())
}
