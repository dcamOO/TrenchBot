use super::common::*;

#[test]
fn rejects_incomplete_wrong_and_duplicate_histories() {
    let mut l = launch(1);
    l.history.complete = false;
    assert!(matches!(
        engine().handle(Event::Launch(l)).unwrap(),
        Outcome::Skipped { .. }
    ));
    let mut l = launch(1);
    l.history.creator = "other".into();
    assert!(matches!(
        engine().handle(Event::Launch(l)).unwrap(),
        Outcome::Skipped { .. }
    ));
    let mut l = launch(1);
    l.history.tokens.push(l.history.tokens[0].clone());
    assert!(engine().handle(Event::Launch(l)).is_err());
    let mut l = launch(1);
    l.history.tokens[0].ath_market_cap_usd = f64::NAN;
    assert!(engine().handle(Event::Launch(l)).is_err());
}

#[test]
fn only_recent_launches_can_be_bought() {
    for (observed_at, allowed) in [(999, false), (1120, true), (1121, false)] {
        let mut l = launch(1);
        l.observed_at = observed_at;
        assert_eq!(
            matches!(
                engine().handle(Event::Launch(l)).unwrap(),
                Outcome::Bought { .. }
            ),
            allowed
        );
    }
}
