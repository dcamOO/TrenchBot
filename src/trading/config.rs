use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradingConfig {
    pub stop_loss_percent: f64,
    pub take_profit_percent: f64,
    /// Decimal string, converted exactly into lamports (no floating point rounding).
    pub sol_per_trade: String,
    pub sniping_enabled: bool,
    pub max_creator_launches: usize,
    pub min_ath_market_cap_usd: f64,
    pub max_launch_age_seconds: u64,
}

impl TradingConfig {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.stop_loss_percent.is_finite()
                && self.stop_loss_percent > 0.0
                && self.stop_loss_percent <= 100.0,
            "stop_loss_percent deve estar entre 0 (exclusivo) e 100"
        );
        ensure!(
            self.take_profit_percent.is_finite() && self.take_profit_percent > 0.0,
            "take_profit_percent deve ser positivo e finito"
        );
        ensure!(
            self.max_creator_launches >= 1,
            "max_creator_launches deve ser >= 1"
        );
        ensure!(
            self.min_ath_market_cap_usd.is_finite() && self.min_ath_market_cap_usd > 0.0,
            "min_ath_market_cap_usd deve ser positivo e finito"
        );
        ensure!(
            self.max_launch_age_seconds > 0,
            "max_launch_age_seconds deve ser > 0"
        );
        self.trade_lamports()?;
        Ok(())
    }

    pub fn trade_lamports(&self) -> Result<u64> {
        let (whole, fraction) = self
            .sol_per_trade
            .split_once('.')
            .unwrap_or((&self.sol_per_trade, ""));
        ensure!(
            !whole.is_empty()
                && whole.bytes().all(|b| b.is_ascii_digit())
                && fraction.bytes().all(|b| b.is_ascii_digit())
                && fraction.len() <= 9,
            "sol_per_trade deve ser decimal positivo com no máximo 9 casas"
        );
        let scale = 10_u64.pow(9 - fraction.len() as u32);
        let whole: u64 = whole.parse()?;
        let fraction: u64 = if fraction.is_empty() {
            0
        } else {
            fraction.parse()?
        };
        let amount = whole
            .checked_mul(1_000_000_000)
            .and_then(|v| v.checked_add(fraction * scale))
            .ok_or_else(|| anyhow::anyhow!("sol_per_trade excede u64 lamports"))?;
        ensure!(amount > 0, "sol_per_trade deve ser pelo menos 1 lamport");
        Ok(amount)
    }
}
