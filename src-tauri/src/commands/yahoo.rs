use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::auth::VaultState;
use crate::error::{Result, VaultError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub isin: String,
    pub ticker: String,
    pub name: String,
    pub price: f64,
    pub currency: String,
    pub change_pct: f64,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    quotes: Option<Vec<SearchResult>>,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    symbol: String,
    #[serde(rename = "longname", default)]
    longname: Option<String>,
    #[serde(rename = "shortname", default)]
    shortname: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    #[serde(rename = "quoteResponse")]
    quote_response: QuoteResponseInner,
}

#[derive(Debug, Deserialize)]
struct QuoteResponseInner {
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

fn http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()
        .map_err(|e| VaultError::Crypto(e.to_string()))
}

async fn isin_to_ticker(client: &reqwest::Client, isin: &str) -> Result<(String, String)> {
    let url = format!(
        "https://query2.finance.yahoo.com/v1/finance/search?q={}&quotesCount=1&newsCount=0&enableFuzzyQuery=false",
        isin
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| VaultError::Crypto(format!("search network error: {e}")))?;

    let body: SearchResponse = resp
        .json()
        .await
        .map_err(|e| VaultError::Crypto(format!("search parse error: {e}")))?;

    let first = body
        .quotes
        .and_then(|q| q.into_iter().next())
        .ok_or_else(|| VaultError::Crypto(format!("no result for ISIN {isin}")))?;

    let name = first.longname.or(first.shortname).unwrap_or_default();
    Ok((first.symbol, name))
}

#[tauri::command]
pub async fn resolve_isin(isin: String, state: State<'_, VaultState>) -> Result<(String, String)> {
    require_unlocked(&state)?;
    let client = http_client()?;
    isin_to_ticker(&client, &isin).await
}

#[tauri::command]
pub async fn fetch_quotes(
    positions: Vec<(String, String)>,
    state: State<'_, VaultState>,
) -> Result<Vec<Quote>> {
    require_unlocked(&state)?;

    if positions.is_empty() {
        return Ok(vec![]);
    }

    let client = http_client()?;
    let symbols: Vec<&str> = positions.iter().map(|(_, t)| t.as_str()).collect();
    let isin_map: std::collections::HashMap<&str, &str> = positions
        .iter()
        .map(|(isin, ticker)| (ticker.as_str(), isin.as_str()))
        .collect();

    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?symbols={}&fields=regularMarketPrice,regularMarketChangePercent,longName,shortName,currency",
        symbols.join(",")
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| VaultError::Crypto(format!("quote network error: {e}")))?;

    let body: QuoteResponse = resp
        .json()
        .await
        .map_err(|e| VaultError::Crypto(format!("quote parse error: {e}")))?;

    let results = body.quote_response.result.unwrap_or_default();

    Ok(results
        .into_iter()
        .filter_map(|r| {
            let price = r.regular_market_price?;
            let isin = isin_map.get(r.symbol.as_str()).unwrap_or(&"").to_string();
            Some(Quote {
                isin,
                ticker: r.symbol,
                name: r.long_name.or(r.short_name).unwrap_or_default(),
                price,
                currency: r.currency.unwrap_or_else(|| "USD".to_string()),
                change_pct: r.regular_market_change_percent.unwrap_or(0.0),
            })
        })
        .collect())
}
