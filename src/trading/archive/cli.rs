use super::{rpc::Rpc, scan::scan, store::Store};
use crate::trading::TradingConfig;
use anyhow::{Result, ensure};

pub fn cache_path() -> String {
    std::env::var("ARCHIVE_DB").unwrap_or_else(|_| "creator-history.sqlite".into())
}

pub fn run(args: &[String]) -> Result<bool> {
    if args.first().map(String::as_str) != Some("--scan-creator") {
        return Ok(false);
    }
    ensure!(args.len() == 3, "uso: --scan-creator CONFIG.json CARTEIRA");
    let config: TradingConfig = serde_json::from_reader(std::fs::File::open(&args[1])?)?;
    config.validate()?;
    args[2].parse::<solana_pubkey::Pubkey>()?;
    let path = cache_path();
    let mut db = Store::open(&path)?;
    let mut rpc = Rpc::from_env(&path)?;
    let report = scan(&mut db, &mut rpc, &args[2], config.max_creator_launches)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(true)
}
