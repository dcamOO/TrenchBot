use super::{
    CreatorHistory, Engine, Event, HistoricalToken, Launch, TradingConfig,
    paper::PaperBroker, pnl::export_csv,
};
use anyhow::Result;

pub fn run_demo() -> Result<()> {
    eprintln!("==================================================");
    eprintln!("🚀 TrenchBot - Demonstração Completa de Simulação");
    eprintln!("==================================================");

    let config = TradingConfig {
        stop_loss_percent: 20.0,
        take_profit_percent: 50.0,
        sol_per_trade: "0.1".into(),
        sniping_enabled: true,
        max_creator_launches: 3,
        min_ath_market_cap_usd: 1_000_000.0,
        max_launch_age_seconds: 120,
    };

    let mut engine = Engine::new(
        config,
        PaperBroker {
            balance_lamports: 10_000_000_000,
        },
    )?;

    eprintln!("\n[1/4] Descoberta no WebSocket PumpPortal...");
    let launch = Launch {
        platform: "pump_fun".into(),
        mint: "DEMO-ROCKET-SOL-PUMP".into(),
        creator: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU".into(),
        launched_at: 1000,
        observed_at: 1005,
        price_sol: 0.000100,
        history: CreatorHistory {
            creator: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU".into(),
            complete: true,
            tokens: vec![HistoricalToken {
                mint: "PRIOR-SOL-TOKEN".into(),
                launched_at: 900,
                ath_market_cap_usd: 2_500_000.0,
            }],
        },
    };
    eprintln!("-> Token: {} | Criador auditado via RPC", launch.mint);

    eprintln!("\n[2/4] Auditoria de Segurança & Google Gemini...");
    eprintln!("-> mint_authority: None | freeze_authority: None");
    eprintln!("-> Parecer Gemini: Veredito SEGURO (Sem risco de rug-pull)");

    eprintln!("\n[3/4] Executando Ordem de Compra (Sniping 0.1 SOL)...");
    let outcome = engine.handle(Event::Launch(launch))?;
    eprintln!("-> Ordem executada: {:?}", outcome);

    if let Ok(telegram) = crate::integration::telegram::Telegram::from_env() {
        let _ = telegram.send_trade_alert(
            crate::integration::telegram::TradeType::Buy,
            "ROCKET",
            1000.0,
            0.015,
            "DEMO_BUY_TX_PUMP_FUN",
        );
        eprintln!("-> 📲 Alerta de COMPRA enviado para o Telegram!");
    }

    eprintln!("\n[4/4] Monitorando Preço em Tempo Real...");
    eprintln!("-> Cotação recebida: 0.000150 SOL (+50.0% Take-Profit)");
    let exit_outcome = engine.handle(Event::Price {
        mint: "DEMO-ROCKET-SOL-PUMP".into(),
        observed_at: 1025,
        price_sol: 0.000150,
    })?;
    eprintln!("-> Posição encerrada: {:?}", exit_outcome);

    if let Ok(telegram) = crate::integration::telegram::Telegram::from_env() {
        let _ = telegram.send_trade_alert(
            crate::integration::telegram::TradeType::Sell,
            "ROCKET",
            1000.0,
            0.0225,
            "DEMO_SELL_TX_PUMP_FUN",
        );
        eprintln!("-> 📲 Alerta de VENDA enviado para o Telegram!");
    }

    let csv = export_csv(engine.pnl_records());
    std::fs::write("demo_pnl.csv", &csv)?;
    eprintln!("\n✅ Demonstração concluída! Relatório salvo em demo_pnl.csv");
    eprintln!("==================================================");
    Ok(())
}
