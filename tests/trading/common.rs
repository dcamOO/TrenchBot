pub(super) use TrenchBot::trading::{
    Broker, CreatorHistory, Engine, Event, HistoricalToken, Launch, Outcome, TradingConfig,
    paper::PaperBroker,
};

pub(super) fn config() -> TradingConfig {
    serde_json::from_str(include_str!("../../examples/trading.json")).unwrap()
}

pub(super) fn launch(successes: usize) -> Launch {
    Launch {
        platform: "pump_fun".into(),
        mint: "new-token".into(),
        creator: "creator".into(),
        launched_at: 1000,
        observed_at: 1010,
        price_sol: 0.001,
        history: CreatorHistory {
            creator: "creator".into(),
            complete: true,
            tokens: (0..successes)
                .map(|i| HistoricalToken {
                    mint: format!("prior-{i}"),
                    launched_at: 100,
                    ath_market_cap_usd: 2_000_000.0,
                })
                .collect(),
        },
    }
}

pub(super) fn engine() -> Engine<PaperBroker> {
    Engine::new(
        config(),
        PaperBroker {
            balance_lamports: 1_000_000_000,
        },
    )
    .unwrap()
}

pub(super) fn price(mint: &str, price_sol: f64, observed_at: u64) -> Event {
    Event::Price {
        mint: mint.into(),
        price_sol,
        observed_at,
    }
}
