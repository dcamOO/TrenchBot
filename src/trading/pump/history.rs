use super::message::PumpMessage;
use crate::trading::{CreatorHistory, Event, HistoricalToken, Launch};
use anyhow::Result;
use std::collections::HashMap;

/// A trusted, complete snapshot must be supplied by a historical data provider.
pub struct Histories(pub HashMap<String, CreatorHistory>);

impl Histories {
    pub fn invalidate(&mut self) {
        // Reconnects may miss deployments; never continue buying on stale counts.
        for history in self.0.values_mut() {
            history.complete = false;
        }
    }

    pub fn event(&mut self, message: PumpMessage, now: u64) -> Result<Event> {
        let price_sol = message.price()?;
        if message.tx_type != "create" {
            return Ok(Event::Price {
                mint: message.mint,
                observed_at: now,
                price_sol,
            });
        }
        let history = self
            .0
            .entry(message.trader_public_key.clone())
            .or_insert_with(|| CreatorHistory {
                creator: message.trader_public_key.clone(),
                complete: false,
                tokens: Vec::new(),
            });
        // Count every observed deployment, including tokens rejected by the strategy.
        let existing = history
            .tokens
            .iter()
            .find(|token| token.mint == message.mint);
        let launched_at = existing.map_or(now, |token| token.launched_at);
        if existing.is_none() {
            history.tokens.push(HistoricalToken {
                mint: message.mint.clone(),
                launched_at,
                ath_market_cap_usd: 0.0,
            });
        }
        Ok(Event::Launch(Launch {
            platform: "pump_fun".into(),
            mint: message.mint,
            creator: message.trader_public_key,
            launched_at,
            observed_at: now,
            price_sol,
            history: history.clone(),
        }))
    }
}
