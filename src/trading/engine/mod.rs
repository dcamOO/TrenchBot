mod history;
mod launch;
mod price;

use super::{TradingConfig, pnl::PnlRecord, types::*};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub struct Engine<B> {
    config: TradingConfig,
    broker: B,
    positions: HashMap<String, Position>,
    bought: HashSet<String>,
    discarded_creators: HashSet<String>,
    pnl_records: Vec<PnlRecord>,
}

fn skipped(reason: &str) -> Outcome {
    Outcome::Skipped {
        reason: reason.into(),
    }
}

impl<B: Broker> Engine<B> {
    pub fn new(config: TradingConfig, broker: B) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config,
            broker,
            positions: HashMap::new(),
            bought: HashSet::new(),
            discarded_creators: HashSet::new(),
            pnl_records: Vec::new(),
        })
    }

    pub fn positions(&self) -> &HashMap<String, Position> {
        &self.positions
    }

    pub fn pnl_records(&self) -> &[PnlRecord] {
        &self.pnl_records
    }

    /// Reconfiguration applies to every open position as well as future buys.
    pub fn configure(&mut self, config: TradingConfig) -> Result<()> {
        config.validate()?;
        self.config = config;
        Ok(())
    }

    pub fn handle(&mut self, event: Event) -> Result<Outcome> {
        match event {
            Event::Launch(launch) => self.launch(launch),
            Event::Price {
                mint,
                observed_at,
                price_sol,
            } => self.price(&mint, observed_at, price_sol),
        }
    }
}
