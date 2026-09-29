use super::common::*;

#[test]
fn keeps_position_inside_limits_and_ignores_out_of_order_prices() {
    let mut e = engine();
    e.handle(Event::Launch(launch(1))).unwrap();
    for p in [0.00080001, 0.001, 0.00149999] {
        assert!(matches!(
            e.handle(price("new-token", p, 1020)).unwrap(),
            Outcome::Skipped { .. }
        ));
    }
    assert!(matches!(
        e.handle(price("new-token", 0.002, 1019)).unwrap(),
        Outcome::Skipped { .. }
    ));
    assert_eq!(e.positions().len(), 1);
    assert!(e.handle(price("new-token", f64::NAN, 1030)).is_err());
    assert!(e.handle(price("new-token", -1.0, 1030)).is_err());
}

#[test]
fn global_changes_affect_open_positions_even_with_sniping_disabled() {
    let mut e = engine();
    e.handle(Event::Launch(launch(1))).unwrap();
    let mut c = config();
    c.take_profit_percent = 10.0;
    c.sniping_enabled = false;
    e.configure(c).unwrap();
    assert!(matches!(
        e.handle(price("new-token", 0.0011, 1020)).unwrap(),
        Outcome::Sold { .. }
    ));
    assert!(matches!(
        e.handle(Event::Launch(launch(1))).unwrap(),
        Outcome::Skipped { .. }
    ));
}
