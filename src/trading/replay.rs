use super::{Engine, Event, TradingConfig, paper::PaperBroker};
use anyhow::Result;
use std::io::{BufRead, BufReader};

pub fn run(config: TradingConfig, events_path: &str, balance: u64) -> Result<()> {
    let mut engine = Engine::new(
        config,
        PaperBroker {
            balance_lamports: balance,
        },
    )?;
    let input: Box<dyn BufRead> = if events_path == "-" {
        Box::new(BufReader::new(std::io::stdin()))
    } else {
        Box::new(BufReader::new(std::fs::File::open(events_path)?))
    };
    for (line_number, line) in input.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let result = serde_json::from_str::<Event>(&line)
            .map_err(anyhow::Error::from)
            .and_then(|event| engine.handle(event));
        match result {
            Ok(outcome) => println!("{}", serde_json::to_string(&outcome)?),
            Err(error) => println!(
                "{}",
                serde_json::json!({"action": "error", "line": line_number + 1, "message": error.to_string()})
            ),
        }
    }
    eprintln!("Posições abertas ao encerrar: {}", engine.positions().len());
    Ok(())
}
