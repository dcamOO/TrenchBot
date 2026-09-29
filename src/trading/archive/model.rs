use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Deployment {
    pub mint: String,
    pub creator: String,
    pub launched_at: u64,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Progress {
    pub anchor: Option<String>,
    pub head: Option<String>,
    pub before: Option<String>,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub creator: String,
    pub complete: bool,
    pub over_limit: bool,
    pub tokens: Vec<Deployment>,
}

/// Implementations must provide mainnet archival history and finalized transactions.
pub trait Archive {
    fn signatures(&mut self, wallet: &str, before: Option<&str>) -> Result<Vec<String>>;
    fn transaction(&mut self, signature: &str) -> Result<Value>;
}
