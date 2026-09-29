use anyhow::{Result, ensure};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PumpMessage {
    pub mint: String,
    pub trader_public_key: String,
    pub tx_type: String,
    pub pool: String,
    #[serde(rename = "vSolInBondingCurve")]
    pub sol_reserves: f64,
    #[serde(rename = "vTokensInBondingCurve")]
    pub token_reserves: f64,
}

impl PumpMessage {
    pub fn parse(text: &str) -> Result<Option<Self>> {
        let value: serde_json::Value = serde_json::from_str(text)?;
        // Subscription acknowledgements and other platforms are not candidates.
        if value.get("mint").is_none() || value["pool"] != "pump" {
            return Ok(None);
        }
        let message: Self = serde_json::from_value(value)?;
        if !matches!(message.tx_type.as_str(), "create" | "buy" | "sell") {
            return Ok(None);
        }
        ensure!(
            !message.mint.is_empty() && !message.trader_public_key.is_empty(),
            "evento sem mint/carteira"
        );
        message.price()?;
        Ok(Some(message))
    }

    pub fn price(&self) -> Result<f64> {
        ensure!(
            self.sol_reserves.is_finite()
                && self.sol_reserves > 0.0
                && self.token_reserves.is_finite()
                && self.token_reserves > 0.0,
            "reservas inválidas"
        );
        let price = self.sol_reserves / self.token_reserves;
        ensure!(price.is_finite() && price > 0.0, "preço inválido");
        Ok(price)
    }
}
