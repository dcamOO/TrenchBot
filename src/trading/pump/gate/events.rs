use super::Gate;
use crate::trading::{CreatorHistory, Event, Launch, pump::message::PumpMessage};
use anyhow::{Result, ensure};

impl Gate {
    pub fn event(&mut self, message: PumpMessage, now: u64) -> Result<Option<Event>> {
        let price_sol = message.price()?;
        if message.tx_type != "create" {
            if let Some(mut launch) = self.ready.remove(&message.mint) {
                // Use a fresh trade after verification, never the pre-scan launch price.
                launch.price_sol = price_sol;
                launch.observed_at = now;
                return Ok(Some(Event::Launch(launch)));
            }
            return Ok(Some(Event::Price {
                mint: message.mint,
                observed_at: now,
                price_sol,
            }));
        }
        if !self.config.sniping_enabled {
            return Ok(None);
        }
        message.mint.parse::<solana_pubkey::Pubkey>()?;
        message.trader_public_key.parse::<solana_pubkey::Pubkey>()?;
        if self.pending.contains_key(&message.mint) || self.ready.contains_key(&message.mint) {
            return Ok(None);
        }
        // A later deployment invalidates an earlier candidate's verified upper bound.
        self.invalidated.extend(
            self.pending
                .iter()
                .filter(|(_, creator)| **creator == message.trader_public_key)
                .map(|(mint, _)| mint.clone()),
        );
        self.invalidated.extend(
            self.ready
                .iter()
                .filter(|(_, launch)| launch.creator == message.trader_public_key)
                .map(|(mint, _)| mint.clone()),
        );
        ensure!(
            self.db.tokens(&message.trader_public_key)?.len() <= self.config.max_creator_launches,
            "token e carteira descartados: cache comprova deployments acima de X"
        );
        ensure!(
            self.pending.len() + self.ready.len() < 16,
            "fila histórica cheia; candidato ignorado"
        );
        let mint = message.mint.clone();
        let creator = message.trader_public_key.clone();
        let history = CreatorHistory {
            creator: message.trader_public_key.clone(),
            complete: false,
            tokens: vec![],
        };
        let launch = Launch {
            platform: "pump_fun".into(),
            mint: message.mint,
            creator: message.trader_public_key,
            launched_at: now,
            observed_at: now,
            price_sol,
            history,
        };
        self.worker
            .jobs
            .try_send(launch)
            .map_err(|_| anyhow::anyhow!("consulta histórica indisponível"))?;
        self.pending.insert(mint, creator);
        Ok(None)
    }
}
