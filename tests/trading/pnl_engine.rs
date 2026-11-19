use super::common::*;
use anyhow::{Result, bail};

fn launch_mint(mint: &str, observed_at: u64) -> Event {
    let mut launch = launch(1);
    launch.mint = mint.into();
    launch.observed_at = observed_at;
    Event::Launch(launch)
}

#[test]
fn stop_loss_exit_records_negative_pnl() {
    let mut e = engine();
    e.handle(launch_mint("A", 1010)).unwrap();
    e.handle(price("A", 0.0008, 1020)).unwrap();
    let record = &e.pnl_records()[0];
    assert_eq!(record.reason, "stop_loss");
    assert_eq!(record.cost_lamports, 100_000_000);
    assert!(record.pnl_lamports < 0);
    assert!((record.pnl_percent - (-20.0)).abs() < 1e-6);
}

#[test]
fn records_keep_closing_order() {
    let mut e = engine();
    e.handle(launch_mint("A", 1010)).unwrap();
    e.handle(launch_mint("B", 1011)).unwrap();
    e.handle(price("B", 0.0015, 1020)).unwrap();
    e.handle(price("A", 0.0008, 1030)).unwrap();
    let mints: Vec<&str> = e.pnl_records().iter().map(|r| r.mint.as_str()).collect();
    assert_eq!(mints, ["B", "A"]);
    assert_eq!(e.pnl_records()[0].opened_at, 1011);
    assert_eq!(e.pnl_records()[1].closed_at, 1030);
}

#[test]
fn price_within_limits_records_nothing() {
    let mut e = engine();
    e.handle(launch_mint("A", 1010)).unwrap();
    e.handle(price("A", 0.0012, 1020)).unwrap();
    assert!(e.pnl_records().is_empty());
    assert_eq!(e.positions().len(), 1);
}

#[test]
fn failed_sell_records_nothing() {
    let mut e = Engine::new(config(), SellFails).unwrap();
    e.handle(launch_mint("A", 1010)).unwrap();
    assert!(e.handle(price("A", 0.0015, 1020)).is_err());
    assert!(e.pnl_records().is_empty());
    assert_eq!(e.positions().len(), 1);
}

#[test]
fn reconfiguration_preserves_closed_records() {
    let mut e = engine();
    e.handle(launch_mint("A", 1010)).unwrap();
    e.handle(price("A", 0.0015, 1020)).unwrap();
    let mut changed = config();
    changed.take_profit_percent = 200.0;
    e.configure(changed).unwrap();
    assert_eq!(e.pnl_records().len(), 1);
    assert_eq!(e.pnl_records()[0].reason, "take_profit");
}

struct SellFails;
impl Broker for SellFails {
    fn buy(&mut self, _: &str, _: u64, _: f64) -> Result<f64> {
        Ok(100.0)
    }
    fn sell(&mut self, _: &str, _: f64, _: f64) -> Result<u64> {
        bail!("sell failed")
    }
}
