use super::common::*;
use anyhow::{Result, bail};

#[test]
fn failed_orders_preserve_state_and_can_be_retried() {
    let mut e = Engine::new(
        config(),
        FailingBroker {
            fail_buy: true,
            fail_sell: true,
        },
    )
    .unwrap();
    assert!(e.handle(Event::Launch(launch(1))).is_err());
    assert!(e.positions().is_empty());
    assert!(matches!(
        e.handle(Event::Launch(launch(1))).unwrap(),
        Outcome::Bought { .. }
    ));
    assert!(e.handle(price("new-token", 0.0015, 1020)).is_err());
    assert_eq!(e.positions().len(), 1);
    assert!(matches!(
        e.handle(price("new-token", 0.0015, 1021)).unwrap(),
        Outcome::Sold { .. }
    ));
}

#[test]
fn insufficient_balance_does_not_open_a_position() {
    let mut e = Engine::new(
        config(),
        PaperBroker {
            balance_lamports: 99_999_999,
        },
    )
    .unwrap();
    assert!(e.handle(Event::Launch(launch(1))).is_err());
    assert!(e.positions().is_empty());
}

struct FailingBroker {
    fail_buy: bool,
    fail_sell: bool,
}
impl Broker for FailingBroker {
    fn buy(&mut self, _: &str, _: u64, _: f64) -> Result<f64> {
        if self.fail_buy {
            self.fail_buy = false;
            bail!("buy failed");
        }
        Ok(100.0)
    }
    fn sell(&mut self, _: &str, _: f64, _: f64) -> Result<u64> {
        if self.fail_sell {
            self.fail_sell = false;
            bail!("sell failed");
        }
        Ok(150_000_000)
    }
}
