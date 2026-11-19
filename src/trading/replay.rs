use super::{Engine, Event, TradingConfig, paper::PaperBroker};
use anyhow::Result;
use std::io::{BufRead, BufReader};

pub fn run(config: TradingConfig, events_path: &str, balance: u64) -> Result<()> {
    run_with_export(config, events_path, balance, None)
}

pub fn run_with_export(
    config: TradingConfig,
    events_path: &str,
    balance: u64,
    export_path: Option<&str>,
) -> Result<()> {
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
    if let Some(path) = export_path {
        let csv = super::pnl::export_csv(engine.pnl_records());
        std::fs::write(path, csv)?;
        eprintln!(
            "Relatório PnL exportado para {path} ({} registros)",
            engine.pnl_records().len()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn config() -> TradingConfig {
        serde_json::from_str(include_str!("../../examples/trading.json")).unwrap()
    }

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("trenchbot-{}-{name}", std::process::id()))
    }

    fn export(events: &str, name: &str) -> Option<String> {
        let events_path = temp_path(&format!("{name}.jsonl"));
        let csv_path = temp_path(&format!("{name}.csv"));
        std::fs::write(&events_path, events).unwrap();
        run_with_export(config(), events_path.to_str().unwrap(), 10_000_000_000, csv_path.to_str())
            .unwrap();
        let csv = std::fs::read_to_string(&csv_path).ok();
        let _ = std::fs::remove_file(events_path);
        let _ = std::fs::remove_file(csv_path);
        csv
    }

    #[test]
    fn exports_closed_trades_from_example_events() {
        let csv = export(include_str!("../../examples/events.jsonl"), "example").unwrap();
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[1].starts_with("DEMO-A,1010,1030,100000000,"));
        assert!(lines[1].ends_with(",take_profit"));
        assert!(lines[2].starts_with("DEMO-B,1050,1060,100000000,"));
        assert!(lines[2].ends_with(",stop_loss"));
    }

    #[test]
    fn malformed_lines_do_not_abort_export() {
        let events = format!("not json\n\n{}", include_str!("../../examples/events.jsonl"));
        let csv = export(&events, "malformed").unwrap();
        assert_eq!(csv.lines().count(), 3);
    }

    #[test]
    fn exports_header_when_no_trade_closes() {
        let csv = export("", "empty").unwrap();
        assert_eq!(csv.lines().count(), 1);
    }

    #[test]
    fn missing_events_file_is_an_error() {
        let missing = temp_path("missing.jsonl");
        assert!(run_with_export(config(), missing.to_str().unwrap(), 1, None).is_err());
    }
}
