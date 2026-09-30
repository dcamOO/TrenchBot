mod events;
mod polling;

use super::history::Histories;
use crate::trading::{
    Launch, TradingConfig,
    archive::{cli::cache_path, store::Store, worker::Worker},
};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Only archival verification can turn a processed stream event into a buy candidate.
pub struct Gate {
    config: TradingConfig,
    db: Store,
    worker: Worker,
    pending: HashMap<String, String>,
    invalidated: HashSet<String>,
    ready: HashMap<String, Launch>,
}

impl Gate {
    pub fn new(config: TradingConfig, ath: Histories) -> Result<Self> {
        Ok(Self {
            db: Store::open(cache_path())?,
            worker: Worker::start(config.clone(), ath)?,
            config,
            pending: HashMap::new(),
            invalidated: HashSet::new(),
            ready: HashMap::new(),
        })
    }

    pub fn invalidate(&mut self) {
        self.invalidated
            .retain(|mint| self.pending.contains_key(mint));
        self.invalidated.extend(self.pending.keys().cloned());
        self.ready.clear();
    }
}
