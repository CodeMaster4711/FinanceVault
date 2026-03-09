use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::commands::auth::VaultState;
use crate::db;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BudgetMonth {
    pub id: String,
    pub month: String,
    pub income: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetSummary {
    pub month: String,
    pub income: f64,
    pub total_expenses: f64,
    pub total_subscriptions_monthly: f64,
    pub remaining: f64,
}

fn require_unlocked(state: &VaultState) -> Result<()> {
    if state.key.lock().unwrap().is_none() {
        return Err(VaultError::Locked);
    }
    Ok(())
}

#[tauri::command]
pub fn get_budget_months(state: State<'_, VaultState>) -> Result<Vec<BudgetMonth>> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let mut stmt = conn
        .prepare("SELECT id, month, income, created_at FROM budget ORDER BY month DESC")
        .map_err(VaultError::Database)?;

    let rows = stmt
        .query_map([], |row| {
            Ok(BudgetMonth {
                id: row.get(0)?,
                month: row.get(1)?,
                income: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(VaultError::Database)?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(VaultError::Database)
}

#[tauri::command]
pub fn upsert_budget_month(
    month: String,
    income: f64,
    state: State<'_, VaultState>,
) -> Result<BudgetMonth> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;
    let now = Utc::now().to_rfc3339();

    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM budget WHERE month = ?1",
            [&month],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        conn.execute(
            "UPDATE budget SET income = ?1 WHERE id = ?2",
            rusqlite::params![income, id],
        )
        .map_err(VaultError::Database)?;
        Ok(BudgetMonth { id, month, income, created_at: now })
    } else {
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO budget (id, month, income, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, month, income, now],
        )
        .map_err(VaultError::Database)?;
        Ok(BudgetMonth { id, month, income, created_at: now })
    }
}

#[tauri::command]
pub fn get_budget_summary(month: String, state: State<'_, VaultState>) -> Result<BudgetSummary> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;

    let income: f64 = conn
        .query_row(
            "SELECT income FROM budget WHERE month = ?1",
            [&month],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    let year_month = month.clone();
    let total_expenses: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM expenses
             WHERE strftime('%Y-%m', date) = ?1",
            [&year_month],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    let total_subscriptions_monthly: f64 = {
        let mut stmt = conn
            .prepare("SELECT amount, billing FROM subscriptions")
            .map_err(VaultError::Database)?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, f64>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(VaultError::Database)?;

        rows.fold(0.0, |acc, r| {
            if let Ok((amount, billing)) = r {
                acc + if billing == "yearly" { amount / 12.0 } else { amount }
            } else {
                acc
            }
        })
    };

    let remaining = income - total_expenses - total_subscriptions_monthly;

    Ok(BudgetSummary {
        month,
        income,
        total_expenses,
        total_subscriptions_monthly,
        remaining,
    })
}
