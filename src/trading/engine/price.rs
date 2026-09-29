use super::*;
use anyhow::ensure;

impl<B: Broker> Engine<B> {
    pub(super) fn price(
        &mut self,
        mint: &str,
        observed_at: u64,
        price_sol: f64,
    ) -> Result<Outcome> {
        ensure!(price_sol.is_finite() && price_sol >= 0.0, "preço inválido");
        let Some(position) = self.positions.get(mint) else {
            return Ok(skipped("sem posição aberta"));
        };
        if observed_at < position.last_observed_at {
            return Ok(skipped("cotação fora de ordem"));
        }
        let value = position.quantity * price_sol;
        ensure!(value.is_finite(), "valor da posição inválido");
        let cost = position.cost_lamports as f64 / 1_000_000_000.0;
        // Account only for floating-point rounding at an inclusive boundary.
        let tolerance = cost * 4.0 * f64::EPSILON;
        let reason = if value <= cost * (1.0 - self.config.stop_loss_percent / 100.0) + tolerance {
            Some("stop_loss")
        } else if value + tolerance >= cost * (1.0 + self.config.take_profit_percent / 100.0) {
            Some("take_profit")
        } else {
            None
        };
        if let Some(reason) = reason {
            let received = self.broker.sell(mint, position.quantity, price_sol)?;
            self.positions.remove(mint);
            Ok(Outcome::Sold {
                mint: mint.into(),
                reason: reason.into(),
                received_lamports: received,
            })
        } else {
            self.positions.get_mut(mint).unwrap().last_observed_at = observed_at;
            Ok(skipped("limites não atingidos"))
        }
    }
}
