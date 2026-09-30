use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoricalToken {
    pub mint: String,
    pub launched_at: u64,
    pub ath_market_cap_usd: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreatorHistory {
    pub creator: String,
    /// Must include all prior launches; an incomplete history cannot prove the upper bound.
    pub complete: bool,
    pub tokens: Vec<HistoricalToken>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Launch {
    pub platform: String,
    pub mint: String,
    pub creator: String,
    pub launched_at: u64,
    pub observed_at: u64,
    pub price_sol: f64,
    pub history: CreatorHistory,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Event {
    Launch(Launch),
    Price {
        mint: String,
        observed_at: u64,
        price_sol: f64,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub mint: String,
    pub quantity: f64,
    pub cost_lamports: u64,
    pub opened_at: u64,
    pub last_observed_at: u64,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Outcome {
    Skipped {
        reason: String,
    },
    Bought {
        mint: String,
        spent_lamports: u64,
        quantity: f64,
    },
    Sold {
        mint: String,
        reason: String,
        received_lamports: u64,
    },
}

/// Implementations must return only confirmed fills. Ambiguous submissions must be
/// reconciled internally before returning; an Err means no trade took place.
pub trait Broker {
    fn buy(&mut self, mint: &str, spend_lamports: u64, price_sol: f64) -> Result<f64>;
    fn sell(&mut self, mint: &str, quantity: f64, price_sol: f64) -> Result<u64>;
}
