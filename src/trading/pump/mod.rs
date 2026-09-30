mod execute;
mod gate;
pub mod history;
pub mod message;
mod socket;

use super::{Engine, Event, TradingConfig, paper::PaperBroker};
use anyhow::Result;
use gate::Gate;
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
pub fn run(config: TradingConfig, histories: Histories, api_key: &str, balance: u64) -> Result<()> {
    let mut gate = Gate::new(config.clone(), histories)?;
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
            gate.invalidate();
        }
        reconnecting = true;
        match Feed::connect(api_key, &positions) {
            Ok(mut feed) => {
                feed.enable_polling()?;
                eprintln!("PumpPortal conectado; ordens SIMULADAS.");
                if session(&mut feed, &mut engine, &mut gate).is_err() {
                    eprintln!("Conexão interrompida; candidatos pendentes invalidados.");
                }
            }
            Err(_) => eprintln!("PumpPortal indisponível; tentando novamente."),
        }
        thread::sleep(Duration::from_secs(delay));
        delay = (delay * 2).min(30);
    }
}

fn session(feed: &mut Feed, engine: &mut Engine<PaperBroker>, gate: &mut Gate) -> Result<()> {
    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        gate.poll(now, |method, keys| feed.subscribe(method, keys))?;
        let incoming = match feed.socket.read() {
            Err(tungstenite::Error::Io(error))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue;
            }
            other => other?,
        };
        let text = match incoming {
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
        let result = (|| -> Result<Option<Event>> {
            let Some(message) = PumpMessage::parse(&text)? else {
                return Ok(None);
            };
            if message.tx_type == "create" && engine.positions().contains_key(&message.mint) {
                return Ok(None);
            }
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            gate.event(message, now)
        })();
        match result {
            Ok(Some(event)) => execute::execute(feed, engine, event)?,
            Ok(None) => {}
            Err(error) => eprintln!("Evento ignorado: {error}"),
        }
    }
}
