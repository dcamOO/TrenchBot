//! Local simulation. No keys, network, real swaps, or Telegram messages.
use super::Broker;
use anyhow::{Result, ensure};

pub struct PaperBroker {
    pub balance_lamports: u64,
}

impl Broker for PaperBroker {
    fn buy(&mut self, _mint: &str, spend_lamports: u64, price_sol: f64) -> Result<f64> {
        ensure!(price_sol.is_finite() && price_sol > 0.0, "preço inválido");
        let quantity = (spend_lamports as f64 / 1_000_000_000.0) / price_sol;
        ensure!(
            quantity.is_finite() && quantity > 0.0,
            "quantidade inválida"
        );
        self.balance_lamports = self
            .balance_lamports
            .checked_sub(spend_lamports)
            .ok_or_else(|| anyhow::anyhow!("saldo SOL insuficiente"))?;
        Ok(quantity)
    }

    fn sell(&mut self, _mint: &str, quantity: f64, price_sol: f64) -> Result<u64> {
        let proceeds = quantity * price_sol * 1_000_000_000.0;
        ensure!(
            quantity.is_finite()
                && quantity > 0.0
                && price_sol.is_finite()
                && price_sol >= 0.0
                && proceeds.is_finite()
                && proceeds >= 0.0
                && proceeds < u64::MAX as f64,
            "valor de venda inválido"
        );
        let proceeds = proceeds.floor() as u64;
        self.balance_lamports = self
            .balance_lamports
            .checked_add(proceeds)
            .ok_or_else(|| anyhow::anyhow!("saldo excede u64"))?;
        Ok(proceeds)
    }
}
