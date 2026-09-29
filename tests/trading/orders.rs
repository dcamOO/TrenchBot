use super::common::*;

#[test]
fn buys_always_spend_the_same_lamports_and_do_not_repeat() {
    let mut e = engine();
    for (mint, p) in [("a", 0.001), ("b", 0.005)] {
        let mut l = launch(1);
        l.mint = mint.into();
        l.price_sol = p;
        assert!(matches!(
            e.handle(Event::Launch(l.clone())).unwrap(),
            Outcome::Bought {
                spent_lamports: 100_000_000,
                ..
            }
        ));
        assert!(matches!(
            e.handle(Event::Launch(l)).unwrap(),
            Outcome::Skipped { .. }
        ));
    }
    assert_eq!(e.positions().len(), 2);
}

#[test]
fn stop_loss_and_take_profit_close_entire_position_at_boundary() {
    for (p, reason) in [
        (0.0008, "stop_loss"),
        (0.0015, "take_profit"),
        (0.0001, "stop_loss"),
        (0.003, "take_profit"),
    ] {
        let mut e = engine();
        e.handle(Event::Launch(launch(1))).unwrap();
        let result = e.handle(price("new-token", p, 1020)).unwrap();
        assert!(matches!(result, Outcome::Sold { reason: r, .. } if r == reason));
        assert!(e.positions().is_empty());
        assert!(matches!(
            e.handle(price("new-token", p, 1021)).unwrap(),
            Outcome::Skipped { .. }
        ));
        assert!(matches!(
            e.handle(Event::Launch(launch(1))).unwrap(),
            Outcome::Skipped { .. }
        ));
    }
}
