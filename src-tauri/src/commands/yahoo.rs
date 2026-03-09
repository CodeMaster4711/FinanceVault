use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::auth::VaultState;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub ticker: String,
    pub name: String,
    pub price: f64,
    pub currency: String,
    pub change_pct: f64,
}

#[derive(Debug, Deserialize)]
struct YahooResponse {
    #[serde(rename = "quoteResponse")]
    quote_response: QuoteResponse,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    result: Option<Vec<YahooResult>>,
}

#[derive(Debug, Deserialize)]
struct YahooResult {
    symbol: String,
    #[serde(rename = "longName", default)]
    long_name: Option<String>,
    #[serde(rename = "shortName", default)]
    short_name: Option<String>,
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,
    currency: Option<String>,
    #[serde(rename = "regularMarketChangePercent")]
    regular_market_change_percent: Option<f64>,
}

fn require_unlocked(state: &VaultState) -> Result<()> {
    if state.key.lock().unwrap().is_none() {
        return Err(VaultError::Locked);
    }
    Ok(())
}

#[tauri::command]
pub async fn fetch_quotes(tickers: Vec<String>, state: State<'_, VaultState>) -> Result<Vec<Quote>> {
    require_unlocked(&state)?;

    if tickers.is_empty() {
        return Ok(vec![]);
    }

    let symbols = tickers.join(",");
    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?symbols={}&fields=regularMarketPrice,regularMarketChangePercent,longName,shortName,currency",
        symbols
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| VaultError::Crypto(format!("network error: {e}")))?;

    let body: YahooResponse = resp
        .json()
        .await
        .map_err(|e| VaultError::Crypto(format!("parse error: {e}")))?;

    let results = body.quote_response.result.unwrap_or_default();

    Ok(results
        .into_iter()
        .filter_map(|r| {
            let price = r.regular_market_price?;
            Some(Quote {
                ticker: r.symbol,
                name: r.long_name.or(r.short_name).unwrap_or_default(),
                price,
                currency: r.currency.unwrap_or_else(|| "USD".to_string()),
                change_pct: r.regular_market_change_percent.unwrap_or(0.0),
            })
        })
        .collect())
}
