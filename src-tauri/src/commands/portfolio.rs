use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::commands::auth::VaultState;
use crate::db;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub id: String,
    pub isin: String,
    pub ticker: String,
    pub name: String,
    pub asset_type: String,
    pub quantity: f64,
    pub avg_buy_price: f64,
    pub currency: String,
    pub country: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePosition {
    pub isin: String,
    pub ticker: String,
    pub name: String,
    pub asset_type: String,
    pub quantity: f64,
    pub avg_buy_price: f64,
    pub currency: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePosition {
    pub quantity: Option<f64>,
    pub avg_buy_price: Option<f64>,
    pub currency: Option<String>,
    pub country: Option<String>,
    pub asset_type: Option<String>,
}

fn require_unlocked(state: &VaultState) -> Result<()> {
    if state.key.lock().unwrap().is_none() {
        return Err(VaultError::Locked);
    }
    Ok(())
}

#[tauri::command]
pub fn get_positions(state: State<'_, VaultState>) -> Result<Vec<Position>> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let mut stmt = conn
        .prepare(
            "SELECT id, isin, ticker, name, asset_type, quantity, avg_buy_price, currency, country, created_at
             FROM portfolio_positions ORDER BY created_at ASC",
        )
        .map_err(VaultError::Database)?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Position {
                id: row.get(0)?,
                isin: row.get(1)?,
                ticker: row.get(2)?,
                name: row.get(3)?,
                asset_type: row.get(4)?,
                quantity: row.get(5)?,
                avg_buy_price: row.get(6)?,
                currency: row.get(7)?,
                country: row.get(8).unwrap_or_default(),
                created_at: row.get(9)?,
            })
        })
        .map_err(VaultError::Database)?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(VaultError::Database)
}

#[tauri::command]
pub fn create_position(input: CreatePosition, state: State<'_, VaultState>) -> Result<Position> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let currency = input.currency.unwrap_or_else(|| "EUR".to_string());
    let country = input.country.unwrap_or_default();

    conn.execute(
        "INSERT INTO portfolio_positions (id, isin, ticker, name, asset_type, quantity, avg_buy_price, currency, country, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        rusqlite::params![
            id, input.isin, input.ticker, input.name, input.asset_type,
            input.quantity, input.avg_buy_price, currency, country, now
        ],
    )
    .map_err(VaultError::Database)?;

    Ok(Position {
        id,
        isin: input.isin,
        ticker: input.ticker,
        name: input.name,
        asset_type: input.asset_type,
        quantity: input.quantity,
        avg_buy_price: input.avg_buy_price,
        currency,
        country,
        created_at: now,
    })
}

#[tauri::command]
pub fn update_position(id: String, input: UpdatePosition, state: State<'_, VaultState>) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;

    if let Some(quantity) = input.quantity {
        conn.execute("UPDATE portfolio_positions SET quantity = ?1 WHERE id = ?2", rusqlite::params![quantity, id])
            .map_err(VaultError::Database)?;
    }
    if let Some(avg_buy_price) = input.avg_buy_price {
        conn.execute("UPDATE portfolio_positions SET avg_buy_price = ?1 WHERE id = ?2", rusqlite::params![avg_buy_price, id])
            .map_err(VaultError::Database)?;
    }
    if let Some(currency) = input.currency {
        conn.execute("UPDATE portfolio_positions SET currency = ?1 WHERE id = ?2", [&currency, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(country) = input.country {
        conn.execute("UPDATE portfolio_positions SET country = ?1 WHERE id = ?2", [&country, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(asset_type) = input.asset_type {
        conn.execute("UPDATE portfolio_positions SET asset_type = ?1 WHERE id = ?2", [&asset_type, &id])
            .map_err(VaultError::Database)?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_position(id: String, state: State<'_, VaultState>) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    conn.execute("DELETE FROM portfolio_positions WHERE id = ?1", [&id])
        .map_err(VaultError::Database)?;
    Ok(())
}
