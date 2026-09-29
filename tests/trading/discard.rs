use super::common::*;

#[test]
fn too_many_deployments_discard_wallet_even_when_ath_is_low_or_history_partial() {
    let mut e = engine();
    let mut l = launch(3);
    l.history.complete = false;
    for t in &mut l.history.tokens {
        t.ath_market_cap_usd = 1.0;
    }
    assert!(
        matches!(e.handle(Event::Launch(l)).unwrap(), Outcome::Skipped { reason } if reason.contains("total de lançamentos"))
    );
    let mut l = launch(1);
    l.mint = "another-token".into();
    assert!(
        matches!(e.handle(Event::Launch(l)).unwrap(), Outcome::Skipped { reason } if reason.contains("carteira descartada"))
    );
    assert!(e.positions().is_empty());
}

#[test]
fn current_token_in_history_is_not_counted_twice() {
    let mut l = launch(2);
    l.history.tokens.push(HistoricalToken {
        mint: l.mint.clone(),
        launched_at: l.launched_at,
        ath_market_cap_usd: 0.0,
    });
    assert!(matches!(
        engine().handle(Event::Launch(l)).unwrap(),
        Outcome::Bought { .. }
    ));
}

#[test]
fn wallet_discard_does_not_prevent_exit_of_existing_position() {
    let mut e = engine();
    e.handle(Event::Launch(launch(1))).unwrap();
    let mut l = launch(3);
    l.mint = "later-token".into();
    assert!(matches!(
        e.handle(Event::Launch(l)).unwrap(),
        Outcome::Skipped { .. }
    ));
    assert!(matches!(
        e.handle(price("new-token", 0.0008, 1030)).unwrap(),
        Outcome::Sold { .. }
    ));
}
