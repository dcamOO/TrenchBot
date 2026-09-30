use super::socket::Feed;
use crate::trading::{Engine, Event, Outcome, paper::PaperBroker};
use anyhow::Result;

pub fn execute(feed: &mut Feed, engine: &mut Engine<PaperBroker>, event: Event) -> Result<()> {
    let candidate = match &event {
        Event::Launch(launch) => Some(launch.mint.clone()),
        _ => None,
    };
    let outcome = engine.handle(event);
    if let Some(mint) = candidate {
        if !matches!(&outcome, Ok(Outcome::Bought { .. }))
            && !engine.positions().contains_key(&mint)
        {
            feed.subscribe("unsubscribeTokenTrade", &[mint])?;
        }
    }
    match outcome {
        Ok(outcome) => {
            println!("{}", serde_json::to_string(&outcome)?);
            if let Outcome::Sold { mint, .. } = outcome {
                feed.subscribe("unsubscribeTokenTrade", &[mint])?;
            }
        }
        Err(error) => eprintln!("Evento ignorado: {error}"),
    }
    Ok(())
}
