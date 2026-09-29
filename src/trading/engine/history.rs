use super::*;
use anyhow::ensure;

impl<B: Broker> Engine<B> {
    pub(super) fn check_creator(&mut self, launch: &Launch) -> Result<Option<Outcome>> {
        if self.discarded_creators.contains(&launch.creator) {
            return Ok(Some(skipped(
                "carteira descartada por excesso de lançamentos",
            )));
        }
        if launch.history.creator != launch.creator {
            return Ok(Some(skipped("histórico de outro criador")));
        }
        // Every deployed mint counts, including the candidate, regardless of ATH.
        // A partial history can already prove that the maximum was exceeded.
        let deployed: HashSet<&str> = launch
            .history
            .tokens
            .iter()
            .map(|token| token.mint.as_str())
            .chain(std::iter::once(launch.mint.as_str()))
            .collect();
        ensure!(
            deployed.iter().all(|mint| !mint.trim().is_empty()),
            "mint ausente no histórico"
        );
        if deployed.len() > self.config.max_creator_launches {
            self.discarded_creators.insert(launch.creator.clone());
            return Ok(Some(skipped(
                "token e carteira descartados: total de lançamentos excede x",
            )));
        }
        if !launch.history.complete {
            return Ok(Some(skipped("histórico incompleto")));
        }
        let mut seen = HashSet::new();
        let mut successful = 0;
        for token in &launch.history.tokens {
            ensure!(
                !token.mint.trim().is_empty()
                    && token.ath_market_cap_usd.is_finite()
                    && token.ath_market_cap_usd >= 0.0,
                "histórico inválido"
            );
            // Reject duplicate records instead of inflating the creator's success count.
            ensure!(seen.insert(&token.mint), "token duplicado no histórico");
            if token.mint != launch.mint
                && token.launched_at < launch.launched_at
                && token.ath_market_cap_usd > self.config.min_ath_market_cap_usd
            {
                successful += 1;
            }
        }
        if successful == 0 {
            return Ok(Some(skipped(
                "criador sem lançamento anterior com ATH acima de y",
            )));
        }
        Ok(None)
    }
}
