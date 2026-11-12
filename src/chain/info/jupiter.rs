use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::{ Deserialize, Serialize };
use super::super::ticker::Asset;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuditInfo {
    #[serde(rename = "mintAuthorityDisabled")]
    pub mint_authority_disabled: Option<bool>,

    #[serde(rename = "freezeAuthorityDisabled")]
    pub freeze_authority_disabled: Option<bool>,

    #[serde(rename = "topHoldersPercentage")]
    pub top_holders_percentage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FirstPoolInfo {
    #[serde(rename = "id")]
    pub address: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_jupiter_json() -> serde_json::Value {
        serde_json::json!({
            "usdPrice": 150.25,
            "liquidity": 500000.0,
            "mcap": 2000000.0,
            "holderCount": 1200,
            "isVerified": true,
            "audit": {
                "mintAuthorityDisabled": true,
                "freezeAuthorityDisabled": false,
                "topHoldersPercentage": 35.5
            },
            "firstPool": {
                "id": "pool123",
                "createdAt": "2026-01-01T12:00:00Z"
            },
            "stats5m": {
                "priceChange": 1.5,
                "buyVolume": 1000.0,
                "sellVolume": 500.0,
                "numBuys": 10,
                "numSells": 5,
                "numTraders": 12
            },
            "stats1h": null,
            "stats6h": null,
            "stats24h": null
        })
    }

    #[test]
    fn deserialize_complete_jupiter_info() {
        let json = valid_jupiter_json();

        let info: JupiterInfo = serde_json::from_value(json).unwrap();

        assert_eq!(info.price_usd, Some(150.25));
        assert_eq!(info.liquidity, Some(500000.0));
        assert_eq!(info.market_cap_usd, Some(2000000.0));
        assert_eq!(info.holder_count, Some(1200));
        assert_eq!(info.verified, Some(true));

        let audit = info.audit.unwrap();
        assert_eq!(audit.mint_authority_disabled, Some(true));
        assert_eq!(audit.freeze_authority_disabled, Some(false));
        assert_eq!(audit.top_holders_percentage, Some(35.5));

        let pool = info.first_pool.unwrap();
        assert_eq!(pool.address, "pool123");
        assert_eq!(pool.created_at, "2026-01-01T12:00:00Z");

        let stats = info.stats_5m.unwrap();
        assert_eq!(stats.price_change, Some(1.5));
        assert_eq!(stats.buy_volume, Some(1000.0));
        assert_eq!(stats.sell_volume, Some(500.0));
        assert_eq!(stats.num_buys, Some(10));
        assert_eq!(stats.num_sells, Some(5));
        assert_eq!(stats.num_traders, Some(12));
    }

    #[test]
    fn deserialize_missing_optional_fields() {
        let json = serde_json::json!({});

        let info: JupiterInfo = serde_json::from_value(json).unwrap();

        assert_eq!(info.price_usd, None);
        assert_eq!(info.liquidity, None);
        assert_eq!(info.market_cap_usd, None);
        assert_eq!(info.holder_count, None);
        assert_eq!(info.verified, None);
        assert!(info.audit.is_none());
        assert!(info.first_pool.is_none());
        assert!(info.stats_5m.is_none());
        assert!(info.stats_1h.is_none());
        assert!(info.stats_6h.is_none());
        assert!(info.stats_24h.is_none());
    }

    #[test]
    fn deserialize_partial_stats() {
        let json = serde_json::json!({
            "buyVolume": 1000.0,
            "sellVolume": 500.0
        });

        let stats: Stats = serde_json::from_value(json).unwrap();

        assert_eq!(stats.buy_volume, Some(1000.0));
        assert_eq!(stats.sell_volume, Some(500.0));
        assert_eq!(stats.price_change, None);
        assert_eq!(stats.num_buys, None);
        assert_eq!(stats.num_sells, None);
        assert_eq!(stats.num_traders, None);
    }

    #[test]
    fn deserialize_partial_audit() {
        let json = serde_json::json!({
            "mintAuthorityDisabled": true
        });

        let audit: AuditInfo = serde_json::from_value(json).unwrap();

        assert_eq!(audit.mint_authority_disabled, Some(true));
        assert_eq!(audit.freeze_authority_disabled, None);
        assert_eq!(audit.top_holders_percentage, None);
    }

    #[test]
    fn deserialize_first_pool() {
        let json = serde_json::json!({
            "id": "pool456",
            "createdAt": "2026-02-10T10:30:00Z"
        });

        let pool: FirstPoolInfo = serde_json::from_value(json).unwrap();

        assert_eq!(pool.address, "pool456");
        assert_eq!(pool.created_at, "2026-02-10T10:30:00Z");
    }

    #[test]
    fn deserialize_invalid_first_pool_fails() {
        let json = serde_json::json!({
            "id": "pool456"
        });

        let result = serde_json::from_value::<FirstPoolInfo>(json);

        assert!(result.is_err());
    }
}
