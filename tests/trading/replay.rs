use super::common::*;

#[test]
fn example_events_run_end_to_end() {
    let mut e = engine();
    let outcomes: Vec<_> = include_str!("../../examples/events.jsonl")
        .lines()
        .map(|line| e.handle(serde_json::from_str(line).unwrap()).unwrap())
        .collect();
    assert!(matches!(&outcomes[0], Outcome::Bought { .. }));
    assert!(matches!(&outcomes[1], Outcome::Skipped { .. }));
    assert!(matches!(&outcomes[2], Outcome::Sold { reason, .. } if reason == "take_profit"));
    assert!(matches!(&outcomes[3], Outcome::Bought { .. }));
    assert!(matches!(&outcomes[4], Outcome::Sold { reason, .. } if reason == "stop_loss"));
    assert!(e.positions().is_empty());
}

#[test]
fn other_platforms_cannot_buy_or_blacklist_a_creator() {
    let mut e = engine();
    let mut l = launch(5);
    l.platform = "other".into();
    assert!(matches!(
        e.handle(Event::Launch(l)).unwrap(),
        Outcome::Skipped { .. }
    ));
    assert!(matches!(
        e.handle(Event::Launch(launch(1))).unwrap(),
        Outcome::Bought { .. }
    ));
}
