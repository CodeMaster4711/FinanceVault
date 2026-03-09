use std::collections::HashMap;

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

// --- Yahoo search response ---

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

// --- Yahoo quote response ---

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

// --- Yahoo v8 chart (history) response ---

#[derive(Debug, Deserialize)]
struct ChartEnvelope {
    chart: ChartInner,
}

#[derive(Debug, Deserialize)]
struct ChartInner {
    result: Option<Vec<ChartResult>>,
}

#[derive(Debug, Deserialize)]
struct ChartResult {
    timestamp: Option<Vec<i64>>,
    indicators: ChartIndicators,
}

#[derive(Debug, Deserialize)]
struct ChartIndicators {
    quote: Option<Vec<ChartQuote>>,
}

#[derive(Debug, Deserialize)]
struct ChartQuote {
    close: Option<Vec<Option<f64>>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PricePoint {
    pub timestamp: i64,
    pub close: f64,
}

// --- Yahoo quoteSummary topHoldings response ---

#[derive(Debug, Deserialize)]
struct QuoteSummaryEnvelope {
    #[serde(rename = "quoteSummary")]
    quote_summary: QuoteSummaryInner,
}

#[derive(Debug, Deserialize)]
struct QuoteSummaryInner {
    result: Option<Vec<QuoteSummaryResult>>,
}

#[derive(Debug, Deserialize)]
struct QuoteSummaryResult {
    #[serde(rename = "topHoldings")]
    top_holdings: Option<TopHoldingsRaw>,
}

#[derive(Debug, Deserialize)]
struct TopHoldingsRaw {
    holdings: Option<Vec<HoldingRaw>>,
    #[serde(rename = "sectorWeightings", default)]
    sector_weightings: Vec<HashMap<String, serde_json::Value>>,
    #[serde(rename = "equityHoldings")]
    equity_holdings: Option<EquityHoldingsRaw>,
}

#[derive(Debug, Deserialize)]
struct HoldingRaw {
    symbol: Option<String>,
    #[serde(rename = "holdingName")]
    holding_name: Option<String>,
    #[serde(rename = "holdingPercent")]
    holding_percent: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct EquityHoldingsRaw {
    #[serde(rename = "priceToEarnings")]
    price_to_earnings: Option<f64>,
    #[serde(rename = "priceToBook")]
    price_to_book: Option<f64>,
    #[serde(rename = "priceToSales")]
    price_to_sales: Option<f64>,
    #[serde(rename = "priceToCashflow")]
    price_to_cashflow: Option<f64>,
}

// --- Public output types ---

#[derive(Debug, Serialize, Clone)]
pub struct FundHolding {
    pub symbol: String,
    pub name: String,
    pub percent: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct SectorWeight {
    pub sector: String,
    pub percent: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct FundData {
    pub ticker: String,
    pub holdings: Vec<FundHolding>,
    pub sector_weights: Vec<SectorWeight>,
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

/// Resolve an ISIN to a Yahoo ticker symbol via the search endpoint.
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

/// Look up a single ISIN: returns ticker + name without fetching price.
#[tauri::command]
pub async fn resolve_isin(isin: String, state: State<'_, VaultState>) -> Result<(String, String)> {
    require_unlocked(&state)?;
    let client = http_client()?;
    isin_to_ticker(&client, &isin).await
}

/// Fetch current quotes for a list of (isin, ticker) pairs.
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

/// Fetch daily closing prices for a ticker. range: "1mo" | "3mo" | "6mo" | "1y" | "2y" | "5y"
#[tauri::command]
pub async fn fetch_history(
    ticker: String,
    range: String,
    state: State<'_, VaultState>,
) -> Result<Vec<PricePoint>> {
    require_unlocked(&state)?;
    let client = http_client()?;

    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?range={}&interval=1d&includePrePost=false",
        ticker, range
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| VaultError::Crypto(format!("history network error: {e}")))?;

    let body: ChartEnvelope = resp
        .json()
        .await
        .map_err(|e| VaultError::Crypto(format!("history parse error: {e}")))?;

    let result = body
        .chart
        .result
        .and_then(|mut r| r.pop())
        .ok_or_else(|| VaultError::Crypto(format!("no history for {ticker}")))?;

    let timestamps = result.timestamp.unwrap_or_default();
    let closes = result
        .indicators
        .quote
        .and_then(|mut q| q.pop())
        .and_then(|q| q.close)
        .unwrap_or_default();

    let points = timestamps
        .into_iter()
        .zip(closes.into_iter())
        .filter_map(|(ts, close)| {
            Some(PricePoint { timestamp: ts, close: close? })
        })
        .collect();

    Ok(points)
}

/// Fetch top holdings and sector weights for a ticker via quoteSummary.
#[tauri::command]
pub async fn fetch_fund_data(
    ticker: String,
    state: State<'_, VaultState>,
) -> Result<FundData> {
    require_unlocked(&state)?;
    let client = http_client()?;

    let url = format!(
        "https://query2.finance.yahoo.com/v10/finance/quoteSummary/{}?modules=topHoldings",
        ticker
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| VaultError::Crypto(format!("fund data network error: {e}")))?;

    let body: QuoteSummaryEnvelope = resp
        .json()
        .await
        .map_err(|e| VaultError::Crypto(format!("fund data parse error: {e}")))?;

    let top = body
        .quote_summary
        .result
        .and_then(|mut r| r.pop())
        .and_then(|r| r.top_holdings)
        .ok_or_else(|| VaultError::Crypto(format!("no fund data for {ticker}")))?;

    let holdings = top
        .holdings
        .unwrap_or_default()
        .into_iter()
        .filter_map(|h| {
            Some(FundHolding {
                symbol: h.symbol.unwrap_or_default(),
                name: h.holding_name.unwrap_or_default(),
                percent: h.holding_percent?,
            })
        })
        .collect();

    let sector_weights = top
        .sector_weightings
        .into_iter()
        .flat_map(|map| {
            map.into_iter().filter_map(|(k, v)| {
                let pct = v.as_f64()?;
                if pct == 0.0 {
                    return None;
                }
                Some(SectorWeight { sector: k, percent: pct })
            })
        })
        .collect();

    Ok(FundData { ticker, holdings, sector_weights })
}
