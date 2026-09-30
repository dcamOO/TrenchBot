use super::*;
use anyhow::ensure;

impl<B: Broker> Engine<B> {
    pub(super) fn launch(&mut self, launch: Launch) -> Result<Outcome> {
        if launch.platform != "pump_fun" {
            return Ok(skipped("lançamento fora da pump.fun"));
        }
        if !self.config.sniping_enabled {
            return Ok(skipped("sniping desativado"));
        }
        ensure!(
            !launch.mint.trim().is_empty() && !launch.creator.trim().is_empty(),
            "mint/criador ausente"
        );
        if let Some(outcome) = self.check_creator(&launch)? {
            return Ok(outcome);
        }
        ensure!(
            launch.price_sol.is_finite() && launch.price_sol > 0.0,
            "preço de compra inválido"
        );
        if self.bought.contains(&launch.mint) {
            return Ok(skipped("token já comprado"));
        }
        let Some(age) = launch.observed_at.checked_sub(launch.launched_at) else {
            return Ok(skipped("lançamento no futuro"));
        };
        if age > self.config.max_launch_age_seconds {
            return Ok(skipped("lançamento antigo"));
        }
        let spent = self.config.trade_lamports()?;
        let quantity = self.broker.buy(&launch.mint, spent, launch.price_sol)?;
        ensure!(
            quantity.is_finite() && quantity > 0.0,
            "broker retornou quantidade inválida"
        );
        self.positions.insert(
            launch.mint.clone(),
            Position {
                mint: launch.mint.clone(),
                quantity,
                cost_lamports: spent,
                opened_at: launch.observed_at,
                last_observed_at: launch.observed_at,
            },
        );
        self.bought.insert(launch.mint.clone());
        Ok(Outcome::Bought {
            mint: launch.mint,
            spent_lamports: spent,
            quantity,
        })
    }
}
