pub mod history;
pub mod message;
mod socket;

use super::{Engine, Outcome, TradingConfig, paper::PaperBroker};
use anyhow::Result;
use history::Histories;
use message::PumpMessage;
use socket::Feed;
use std::{
    collections::HashSet,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tungstenite::Message;

/// Live PumpPortal data, simulated fills. Uses one connection for all subscriptions.
pub fn run(
    config: TradingConfig,
    mut histories: Histories,
    api_key: &str,
    balance: u64,
) -> Result<()> {
    let mut engine = Engine::new(
        config,
        PaperBroker {
            balance_lamports: balance,
        },
    )?;
    let mut delay = 1;
    let mut reconnecting = false;
    loop {
        let positions: HashSet<_> = engine.positions().keys().cloned().collect();
        if reconnecting {
            histories.invalidate();
        }
        reconnecting = true;
        match Feed::connect(api_key, &positions) {
            Ok(mut feed) => {
                eprintln!("PumpPortal conectado; ordens SIMULADAS.");
                if session(&mut feed, &mut engine, &mut histories).is_err() {
                    eprintln!("Conexão interrompida; histórico invalidado para novas compras.");
                }
            }
            Err(_) => eprintln!("PumpPortal indisponível; tentando novamente."),
        }
        thread::sleep(Duration::from_secs(delay));
        delay = (delay * 2).min(30);
    }
}

fn session(
    feed: &mut Feed,
    engine: &mut Engine<PaperBroker>,
    histories: &mut Histories,
) -> Result<()> {
    loop {
        let text = match feed.socket.read()? {
            Message::Text(text) => text,
            Message::Close(_) => anyhow::bail!("conexão encerrada"),
            Message::Ping(_) => {
                feed.socket.flush()?;
                continue;
            }
            _ => continue,
        };
        // A server-side subscription error means prices may be unavailable.
        let value: serde_json::Value = serde_json::from_str(&text)?;
        anyhow::ensure!(
            value.get("errors").is_none() && value.get("error").is_none(),
            "assinatura recusada"
        );
        let result = (|| -> Result<Option<Outcome>> {
            let Some(message) = PumpMessage::parse(&text)? else {
                return Ok(None);
            };
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            Ok(Some(engine.handle(histories.event(message, now)?)?))
        })();
        match result {
            Ok(Some(outcome)) => {
                println!("{}", serde_json::to_string(&outcome)?);
                match outcome {
                    Outcome::Bought { mint, .. } => {
                        feed.subscribe("subscribeTokenTrade", &[mint])?
                    }
                    Outcome::Sold { mint, .. } => {
                        feed.subscribe("unsubscribeTokenTrade", &[mint])?
                    }
                    _ => {}
                }
            }
            Ok(None) => {}
            Err(error) => eprintln!("Evento ignorado: {error}"),
        }
    }
}
