use anyhow::Result;
use reqwest::header::*;
use serde::Deserialize;

const COINCAP_API_KEY: &str = env!("COINCAP_API_KEY");
const COINCAP_ENDPOINT: &str = "https://api.coincap.io/v2";

#[derive(Debug, Deserialize)]
struct CoinCapRate {
    data: CoinCapRateData,
}

#[derive(Debug, Deserialize)]
struct CoinCapRateData {
    id: String,
    symbol: String,
    timestamp: Option<usize>,
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "currencySymbol")]
    currency_symbol: Option<String>,
    #[serde(rename = "rateUsd")]
    rate_usd: String,
}

/// Fetch the ETH/USD exchange rate.
pub async fn eth_usd() -> Result<()> {
    let url = format!("{}/rates/ethereum", COINCAP_ENDPOINT);
    let client = reqwest::Client::builder().build()?;
    let result = client
        .get(&url)
        .bearer_auth(COINCAP_API_KEY)
        .send()
        .await?
        .json::<CoinCapRate>()
        .await?;
    Ok(())
}
