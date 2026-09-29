use super::common::*;

#[test]
fn creator_must_have_success_and_total_launches_including_candidate_within_x() {
    for n in 0..=4 {
        let outcome = engine().handle(Event::Launch(launch(n))).unwrap();
        assert_eq!(
            matches!(outcome, Outcome::Bought { .. }),
            (1..=2).contains(&n)
        );
    }
}

#[test]
fn ath_must_strictly_exceed_y_and_only_prior_tokens_count() {
    let mut l = launch(1);
    l.history.tokens[0].ath_market_cap_usd = 1_000_000.0;
    assert!(matches!(
        engine().handle(Event::Launch(l)).unwrap(),
        Outcome::Skipped { .. }
    ));
    let mut l = launch(1);
    l.history.tokens[0].mint = l.mint.clone();
    assert!(matches!(
        engine().handle(Event::Launch(l)).unwrap(),
        Outcome::Skipped { .. }
    ));
    let mut l = launch(1);
    l.history.tokens[0].launched_at = l.launched_at;
    assert!(matches!(
        engine().handle(Event::Launch(l)).unwrap(),
        Outcome::Skipped { .. }
    ));
    let mut l = launch(1);
    l.history.tokens.push(HistoricalToken {
        mint: "failed".into(),
        launched_at: 100,
        ath_market_cap_usd: 20.0,
    });
    assert!(matches!(
        engine().handle(Event::Launch(l)).unwrap(),
        Outcome::Bought { .. }
    ));
}
