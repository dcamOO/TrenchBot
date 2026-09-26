mod chain;
mod integration;

use anyhow::Result;

use chain::{
    analysis::{
        gemini::Gemini,
        processing::process,
    },
    ticker::Asset,
};

use integration::telegram::{
    Telegram,
    TradeType,
};

fn main() -> Result<()> {
    println!("=== TrenchBot integration test ===");

    let ticker = std::env::var("TEST_TICKER")
        .unwrap_or_else(|_| "BONK".to_string());

    let rpc_url = std::env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| {
            "https://api.mainnet-beta.solana.com".to_string()
        });

    let jupiter_key =
        std::env::var("JUPITER_API_KEY")?;

    println!("\n[1/4] Resolving ticker...");
    println!("Ticker: {ticker}");

    let asset = Asset::new(
        &ticker,
        &rpc_url,
        &jupiter_key,
    )?;

    println!("Mint: {}", asset.mint);

    println!("\n[2/4] Collecting token information...");

    let info = asset.get_info(&jupiter_key)?;

    println!("{info:#?}");

    println!("\n[3/4] Processing token information...");

    let processed = process(info);

    println!("{processed:#?}");

    if let Ok(gemini_key) =
        std::env::var("GEMINI_API_KEY")
    {
        println!("\n[4/4] Testing Gemini...");

        let gemini = Gemini::new(gemini_key);

        let analysis = gemini.analyze(&processed)?;

        println!("\n=== Gemini analysis ===\n");
        println!("{analysis}");
    } else {
        println!(
            "\n[4/4] GEMINI_API_KEY not set; skipping Gemini."
        );
    }

    println!("\n[Telegram] Testing trade alert...");

    let telegram = Telegram::from_env()?;

    telegram.send_trade_alert(
        TradeType::Buy,
        &asset.ticker,
        1_000.0,
        0.000001,
        "TEST_SIGNATURE_NOT_A_REAL_TRANSACTION",
    )?;

    println!("Telegram alert sent.");

    println!("\n=== Test completed ===");

    Ok(())
}
