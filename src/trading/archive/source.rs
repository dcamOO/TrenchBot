use super::{model::Archive, rpc::Rpc};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::collections::HashSet;

impl Archive for Rpc {
    fn signatures(&mut self, wallet: &str, before: Option<&str>) -> Result<Vec<String>> {
        wallet.parse::<solana_pubkey::Pubkey>()?;
        let mut options = json!({"limit":1000,"commitment":"finalized"});
        if let Some(before) = before {
            options["before"] = json!(before);
        }
        let response = self.call("getSignaturesForAddress", json!([wallet, options]))?;
        let rows = response.as_array().context("página RPC inválida")?;
        ensure!(rows.len() <= 1000, "página RPC excedeu limite");
        let mut seen = HashSet::new();
        rows.iter()
            .map(|row| {
                ensure!(
                    row["confirmationStatus"] == "finalized",
                    "assinatura não finalizada"
                );
                let signature = row["signature"].as_str().context("assinatura ausente")?;
                ensure!(
                    bs58::decode(signature).into_vec()?.len() == 64,
                    "assinatura inválida"
                );
                ensure!(
                    Some(signature) != before && seen.insert(signature),
                    "paginação repetida"
                );
                Ok(signature.to_owned())
            })
            .collect()
    }

    fn transaction(&mut self, signature: &str) -> Result<Value> {
        self.call("getTransaction", json!([signature, {
            "encoding":"jsonParsed", "commitment":"finalized", "maxSupportedTransactionVersion":0
        }]))
    }
}
