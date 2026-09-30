use super::{model::Archive, rpc::now};
use anyhow::{Result, ensure};
use serde_json::Value;

/// Bounds each live candidate's work without losing the durable scan cursor.
pub struct Deadline<'a, A> {
    pub source: &'a mut A,
    pub expires_at: u64,
}

impl<A: Archive> Archive for Deadline<'_, A> {
    fn signatures(&mut self, wallet: &str, before: Option<&str>) -> Result<Vec<String>> {
        ensure!(
            now()? <= self.expires_at,
            "candidato expirou durante consulta histórica"
        );
        self.source.signatures(wallet, before)
    }

    fn transaction(&mut self, signature: &str) -> Result<Value> {
        ensure!(
            now()? <= self.expires_at,
            "candidato expirou durante consulta histórica"
        );
        self.source.transaction(signature)
    }
}
