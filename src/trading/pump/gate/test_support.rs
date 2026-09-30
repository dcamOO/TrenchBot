use super::*;
use crate::trading::{CreatorHistory, Event, pump::message::PumpMessage};
use std::sync::mpsc::{self, Receiver, SyncSender};

pub fn key(byte: u8) -> String {
    bs58::encode([byte; 32]).into_string()
}

pub fn message(mint: u8, kind: &str) -> PumpMessage {
    PumpMessage {
        mint: key(mint),
        trader_public_key: key(1),
        tx_type: kind.into(),
        pool: "pump".into(),
        sol_reserves: 30.0,
        token_reserves: 1000.0,
    }
}

pub fn launch(mint: u8) -> Launch {
    Launch {
        platform: "pump_fun".into(),
        mint: key(mint),
        creator: key(1),
        launched_at: 100,
        observed_at: 100,
        price_sol: 1.0,
        history: CreatorHistory {
            creator: key(1),
            complete: true,
            tokens: vec![],
        },
    }
}

pub fn gate() -> (Gate, Receiver<Launch>, SyncSender<(String, Result<Launch>)>) {
    let (jobs, incoming) = mpsc::sync_channel(16);
    let (outgoing, results) = mpsc::sync_channel(16);
    let config = TradingConfig {
        stop_loss_percent: 20.0,
        take_profit_percent: 50.0,
        sol_per_trade: "0.1".into(),
        sniping_enabled: true,
        max_creator_launches: 3,
        min_ath_market_cap_usd: 1_000_000.0,
        max_launch_age_seconds: 120,
    };
    (
        Gate {
            config,
            db: Store::open(":memory:").unwrap(),
            worker: Worker { jobs, results },
            pending: HashMap::new(),
            invalidated: HashSet::new(),
            ready: HashMap::new(),
        },
        incoming,
        outgoing,
    )
}

pub fn is_price(event: Option<Event>) -> bool {
    matches!(event, Some(Event::Price { .. }))
}
