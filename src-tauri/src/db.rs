use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

pub fn open(db_path: &Path) -> SqlResult<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch("
        PRAGMA journal_mode=WAL;
        PRAGMA foreign_keys=ON;
    ")?;
    Ok(conn)
}

pub fn migrate(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS vault_meta (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS expenses (
            id          TEXT PRIMARY KEY,
            title       TEXT NOT NULL,
            amount      REAL NOT NULL,
            currency    TEXT NOT NULL DEFAULT 'EUR',
            category    TEXT NOT NULL,
            date        TEXT NOT NULL,
            created_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS subscriptions (
            id           TEXT PRIMARY KEY,
            name         TEXT NOT NULL,
            amount       REAL NOT NULL,
            currency     TEXT NOT NULL DEFAULT 'EUR',
            billing      TEXT NOT NULL CHECK(billing IN ('monthly','yearly')),
            next_billing TEXT NOT NULL,
            created_at   TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS portfolio_positions (
            id             TEXT PRIMARY KEY,
            ticker         TEXT NOT NULL,
            isin           TEXT,
            name           TEXT NOT NULL,
            asset_type     TEXT NOT NULL CHECK(asset_type IN ('stock','etf','crypto','other')),
            quantity       REAL NOT NULL,
            avg_buy_price  REAL NOT NULL,
            currency       TEXT NOT NULL DEFAULT 'EUR',
            country        TEXT,
            created_at     TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS budget (
            id         TEXT PRIMARY KEY,
            month      TEXT NOT NULL,
            income     REAL NOT NULL,
            created_at TEXT NOT NULL
        );
    ")?;
    Ok(())
}
