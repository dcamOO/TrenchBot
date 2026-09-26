use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

use super::super::ticker::Asset;

#[derive(Debug, Deserialize)]
pub(crate) struct JupiterInfo {
    #[serde(rename = "usdPrice")]
    pub price_usd: Option<f64>,

    pub liquidity: Option<f64>,

    #[serde(rename = "mcap")]
    pub market_cap_usd: Option<f64>,

    #[serde(rename = "holderCount")]
    pub holder_count: Option<u64>,

    #[serde(rename = "isVerified")]
    pub verified: Option<bool>,

    pub audit: Option<AuditInfo>,

    #[serde(rename = "firstPool")]
    pub first_pool: Option<FirstPoolInfo>,

    #[serde(rename = "stats5m")]
    pub stats_5m: Option<Stats>,

    #[serde(rename = "stats1h")]
    pub stats_1h: Option<Stats>,

    #[serde(rename = "stats6h")]
    pub stats_6h: Option<Stats>,

    #[serde(rename = "stats24h")]
    pub stats_24h: Option<Stats>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuditInfo {
    #[serde(rename = "mintAuthorityDisabled")]
    pub mint_authority_disabled: Option<bool>,

    #[serde(rename = "freezeAuthorityDisabled")]
    pub freeze_authority_disabled: Option<bool>,

    #[serde(rename = "topHoldersPercentage")]
    pub top_holders_percentage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FirstPoolInfo {
    #[serde(rename = "id")]
    pub address: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Stats {
    #[serde(rename = "priceChange")]
    pub price_change: Option<f64>,

    #[serde(rename = "buyVolume")]
    pub buy_volume: Option<f64>,

    #[serde(rename = "sellVolume")]
    pub sell_volume: Option<f64>,

    #[serde(rename = "numBuys")]
    pub num_buys: Option<u64>,

    #[serde(rename = "numSells")]
    pub num_sells: Option<u64>,

    #[serde(rename = "numTraders")]
    pub num_traders: Option<u64>,
}

impl Asset {
    pub(crate) fn get_jupiter_info(
        &self,
        api_key: &str,
    ) -> Result<JupiterInfo> {
        let response = Client::new()
            .get("https://api.jup.ag/tokens/v2/search")
            .query(&[("query", self.mint.to_string())])
            .header("x-api-key", api_key)
            .send()?
            .error_for_status()?
            .json::<Vec<JupiterInfo>>()?;

        response
            .into_iter()
            .next()
            .ok_or_else(|| {
                anyhow!("Mint not found on Jupiter: {}", self.mint)
            })
    }
}
