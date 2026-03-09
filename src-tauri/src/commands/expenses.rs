use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::commands::auth::VaultState;
use crate::db;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub id: String,
    pub title: String,
    pub amount: f64,
    pub currency: String,
    pub category: String,
    pub date: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateExpense {
    pub title: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub category: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExpense {
    pub title: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub category: Option<String>,
    pub date: Option<String>,
}

fn require_unlocked(state: &VaultState) -> Result<()> {
    if state.key.lock().unwrap().is_none() {
        return Err(VaultError::Locked);
    }
    Ok(())
}

#[tauri::command]
pub fn get_expenses(state: State<'_, VaultState>) -> Result<Vec<Expense>> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, amount, currency, category, date, created_at
             FROM expenses ORDER BY date DESC",
        )
        .map_err(VaultError::Database)?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Expense {
                id: row.get(0)?,
                title: row.get(1)?,
                amount: row.get(2)?,
                currency: row.get(3)?,
                category: row.get(4)?,
                date: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(VaultError::Database)?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(VaultError::Database)
}

#[tauri::command]
pub fn create_expense(input: CreateExpense, state: State<'_, VaultState>) -> Result<Expense> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let currency = input.currency.unwrap_or_else(|| "EUR".to_string());

    conn.execute(
        "INSERT INTO expenses (id, title, amount, currency, category, date, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![id, input.title, input.amount, currency, input.category, input.date, now],
    )
    .map_err(VaultError::Database)?;

    Ok(Expense {
        id,
        title: input.title,
        amount: input.amount,
        currency,
        category: input.category,
        date: input.date,
        created_at: now,
    })
}

#[tauri::command]
pub fn update_expense(id: String, input: UpdateExpense, state: State<'_, VaultState>) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;

    if let Some(title) = input.title {
        conn.execute("UPDATE expenses SET title = ?1 WHERE id = ?2", [&title, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(amount) = input.amount {
        conn.execute("UPDATE expenses SET amount = ?1 WHERE id = ?2", rusqlite::params![amount, id])
            .map_err(VaultError::Database)?;
    }
    if let Some(currency) = input.currency {
        conn.execute("UPDATE expenses SET currency = ?1 WHERE id = ?2", [&currency, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(category) = input.category {
        conn.execute("UPDATE expenses SET category = ?1 WHERE id = ?2", [&category, &id])
            .map_err(VaultError::Database)?;
    }
    if let Some(date) = input.date {
        conn.execute("UPDATE expenses SET date = ?1 WHERE id = ?2", [&date, &id])
            .map_err(VaultError::Database)?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_expense(id: String, state: State<'_, VaultState>) -> Result<()> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    conn.execute("DELETE FROM expenses WHERE id = ?1", [&id])
        .map_err(VaultError::Database)?;
    Ok(())
}
