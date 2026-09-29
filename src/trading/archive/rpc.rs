use super::store::Store;
use anyhow::{Context, Result, ensure};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::{
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

pub struct Rpc {
    client: Client,
    url: reqwest::Url,
    budget: Store,
    daily_limit: u64,
    last_request: Option<Instant>,
}

impl Rpc {
    pub fn from_env(path: impl AsRef<Path>) -> Result<Self> {
        let key = std::env::var("HELIUS_API_KEY").context("HELIUS_API_KEY ausente")?;
        ensure!(!key.trim().is_empty(), "HELIUS_API_KEY vazia");
        let daily_limit: u64 = std::env::var("ARCHIVE_DAILY_REQUESTS")
            .unwrap_or_else(|_| "25000".into())
            .parse()?;
        ensure!(daily_limit > 0, "orçamento diário deve ser positivo");
        // Helius archival mainnet only: a pruned public RPC cannot prove completeness.
        let mut url = reqwest::Url::parse("https://mainnet.helius-rpc.com/")?;
        url.query_pairs_mut().append_pair("api-key", &key);
        Ok(Self {
            client: Client::builder()
                .timeout(Duration::from_secs(20))
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
            url,
            budget: Store::open(path)?,
            daily_limit,
            last_request: None,
        })
    }

    pub(super) fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        if let Some(last) = self.last_request {
            std::thread::sleep(Duration::from_millis(150).saturating_sub(last.elapsed()));
        }
        self.budget.reserve_request(now()?, self.daily_limit)?;
        self.last_request = Some(Instant::now());
        // Never propagate credential-bearing reqwest URLs or provider error bodies.
        let response = self
            .client
            .post(self.url.clone())
            .json(&json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}))
            .send()
            .map_err(|_| anyhow::anyhow!("RPC indisponível"))?;
        ensure!(
            response.status().is_success(),
            "RPC HTTP {}",
            response.status().as_u16()
        );
        let value: Value = response
            .json()
            .map_err(|_| anyhow::anyhow!("resposta RPC inválida"))?;
        ensure!(value.get("error").is_none(), "RPC recusou {method}");
        value.get("result").cloned().context("RPC sem result")
    }
}
