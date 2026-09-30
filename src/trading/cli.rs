use super::{TradingConfig, pump, replay};
use anyhow::{Context, Result, ensure};

pub fn run() -> Result<bool> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if super::archive::cli::run(&args)? {
        return Ok(true);
    }
    if args.is_empty() {
        return Ok(false);
    }
    ensure!(
        args.len() == 3 && matches!(args[0].as_str(), "--paper" | "--pump-paper"),
        "uso: TrenchBot --paper CONFIG.json EVENTOS.jsonl | --pump-paper CONFIG.json HISTORICOS.json"
    );
    let config: TradingConfig = serde_json::from_reader(std::fs::File::open(&args[1])?)
        .context("configuração de trading inválida")?;
    config.validate()?;
    let balance: u64 = std::env::var("PAPER_BALANCE_LAMPORTS")
        .unwrap_or_else(|_| "10000000000".into())
        .parse()
        .context("PAPER_BALANCE_LAMPORTS deve ser um inteiro u64")?;
    eprintln!("SIMULAÇÃO: saldo {balance} lamports; nenhuma transação real será enviada.");
    if args[0] == "--paper" {
        replay::run(config, &args[2], balance)?;
    } else {
        let api_key = std::env::var("PUMPPORTAL_API_KEY").context("PUMPPORTAL_API_KEY ausente")?;
        ensure!(!api_key.trim().is_empty(), "PUMPPORTAL_API_KEY vazia");
        let histories =
            pump::history::Histories(serde_json::from_reader(std::fs::File::open(&args[2])?)?);
        eprintln!(
            "Dados de trades do PumpPortal são tarifados pelo provedor, mesmo com ordens simuladas."
        );
        pump::run(config, histories, &api_key, balance)?;
    }
    Ok(true)
}
