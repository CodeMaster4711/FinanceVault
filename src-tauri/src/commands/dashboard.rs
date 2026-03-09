use serde::Serialize;
use tauri::State;

use crate::commands::auth::VaultState;
use crate::db;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub budget_income: f64,
    pub budget_expenses: f64,
    pub budget_subscriptions: f64,
    pub budget_remaining: f64,
    pub expense_count_this_month: i64,
    pub subscriptions_count: i64,
    pub savings_plans_count: i64,
    pub savings_plans_monthly: f64,
    pub portfolio_positions: i64,
    pub portfolio_invested: f64,
}

fn require_unlocked(state: &VaultState) -> Result<()> {
    if state.key.lock().unwrap().is_none() {
        return Err(VaultError::Locked);
    }
    Ok(())
}

#[tauri::command]
pub fn get_dashboard_summary(state: State<'_, VaultState>) -> Result<DashboardSummary> {
    require_unlocked(&state)?;
    let conn = db::open(&state.db_path)?;

    let current_month = chrono::Utc::now().format("%Y-%m").to_string();

    let budget_income: f64 = conn
        .query_row(
            "SELECT COALESCE(income, 0) FROM budget WHERE month = ?1",
            [&current_month],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    let budget_expenses: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM expenses WHERE strftime('%Y-%m', date) = ?1",
            [&current_month],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    let budget_subscriptions: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(CASE billing WHEN 'monthly' THEN amount WHEN 'yearly' THEN amount/12.0 ELSE 0 END), 0) FROM subscriptions",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    let expense_count_this_month: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM expenses WHERE strftime('%Y-%m', date) = ?1",
            [&current_month],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let subscriptions_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM subscriptions", [], |r| r.get(0))
        .unwrap_or(0);

    let savings_plans_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM savings_plans", [], |r| r.get(0))
        .unwrap_or(0);

    let savings_plans_monthly: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(CASE interval WHEN 'monthly' THEN amount WHEN 'quarterly' THEN amount/3.0 WHEN 'yearly' THEN amount/12.0 ELSE 0 END), 0) FROM savings_plans",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    let portfolio_positions: i64 = conn
        .query_row("SELECT COUNT(*) FROM portfolio_positions", [], |r| r.get(0))
        .unwrap_or(0);

    let portfolio_invested: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(quantity * avg_buy_price), 0) FROM portfolio_positions",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    Ok(DashboardSummary {
        budget_income,
        budget_expenses,
        budget_subscriptions,
        budget_remaining: budget_income - budget_expenses - budget_subscriptions,
        expense_count_this_month,
        subscriptions_count,
        savings_plans_count,
        savings_plans_monthly,
        portfolio_positions,
        portfolio_invested,
    })
}
