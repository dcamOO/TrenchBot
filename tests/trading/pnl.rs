use TrenchBot::trading::pnl::{export_csv, PnlRecord};

#[test]
fn calculates_positive_pnl_correctly() {
    let record = PnlRecord::new(
        "MINT1".into(),
        1000,
        1050,
        100_000_000,
        120_000_000,
        "take_profit".into(),
    );
    assert_eq!(record.pnl_lamports, 20_000_000);
    assert!((record.pnl_sol - 0.02).abs() < 1e-9);
    assert!((record.pnl_percent - 20.0).abs() < 1e-9);
    assert_eq!(record.reason, "take_profit");
}

#[test]
fn calculates_negative_pnl_correctly() {
    let record = PnlRecord::new(
        "MINT2".into(),
        2000,
        2020,
        200_000_000,
        170_000_000,
        "stop_loss".into(),
    );
    assert_eq!(record.pnl_lamports, -30_000_000);
    assert!((record.pnl_sol - (-0.03)).abs() < 1e-9);
    assert!((record.pnl_percent - (-15.0)).abs() < 1e-9);
    assert_eq!(record.reason, "stop_loss");
}

#[test]
fn formats_pnl_csv_properly() {
    let records = vec![
        PnlRecord::new("MINT1".into(), 100, 200, 100_000_000, 110_000_000, "take_profit".into()),
        PnlRecord::new("MINT2".into(), 300, 400, 100_000_000, 90_000_000, "stop_loss".into()),
    ];
    let csv = export_csv(&records);
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(
        lines[0],
        "mint,opened_at,closed_at,cost_lamports,received_lamports,pnl_lamports,pnl_sol,pnl_percent,reason"
    );
    assert!(lines[1].contains("MINT1,100,200,100000000,110000000,10000000,0.010000,10.00,take_profit"));
    assert!(lines[2].contains("MINT2,300,400,100000000,90000000,-10000000,-0.010000,-10.00,stop_loss"));
}

#[test]
fn engine_records_pnl_on_closed_trades() {
    use super::common::*;
    let mut engine = engine();
    engine.handle(Event::Launch(launch(1))).unwrap();
    assert_eq!(engine.positions().len(), 1);
    assert_eq!(engine.pnl_records().len(), 0);

    let outcome = engine.handle(price("new-token", 0.0015, 1020)).unwrap();
    assert!(matches!(outcome, Outcome::Sold { .. }));
    assert_eq!(engine.positions().len(), 0);
    assert_eq!(engine.pnl_records().len(), 1);

    let record = &engine.pnl_records()[0];
    assert_eq!(record.mint, "new-token");
    assert_eq!(record.opened_at, 1010);
    assert_eq!(record.closed_at, 1020);
    assert_eq!(record.reason, "take_profit");
    assert!(record.pnl_lamports > 0);
    assert!(record.pnl_percent > 49.9);
}
