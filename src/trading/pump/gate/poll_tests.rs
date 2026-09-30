use super::test_support::*;

#[test]
fn late_results_after_reconnect_cannot_approve_a_candidate() {
    let (mut gate, _jobs, results) = gate();
    gate.pending.insert(key(2), key(1));
    gate.ready.insert(key(3), launch(3));
    gate.invalidate();
    results.send((key(2), Ok(launch(2)))).unwrap();
    gate.poll(150, |_, _| panic!("stale candidate must not subscribe"))
        .unwrap();
    assert!(gate.ready.is_empty());
    assert!(gate.pending.is_empty());
    assert!(gate.invalidated.is_empty());
}

#[test]
fn subscribes_only_verified_candidates_and_unsubscribes_on_expiry() {
    let (mut gate, _jobs, results) = gate();
    gate.pending.insert(key(2), key(1));
    results.send((key(2), Ok(launch(2)))).unwrap();
    let mut subscriptions = vec![];
    gate.poll(150, |method, keys| {
        subscriptions.push((method.to_string(), keys.to_vec()));
        Ok(())
    })
    .unwrap();
    assert_eq!(
        subscriptions,
        [("subscribeTokenTrade".to_string(), vec![key(2)])]
    );
    assert!(gate.ready.contains_key(&key(2)));
    gate.poll(221, |method, keys| {
        subscriptions.push((method.to_string(), keys.to_vec()));
        Ok(())
    })
    .unwrap();
    assert_eq!(
        subscriptions[1],
        ("unsubscribeTokenTrade".to_string(), vec![key(2)])
    );
    assert!(gate.ready.is_empty());
}

#[test]
fn invalidated_ready_candidate_is_unsubscribed_before_price_processing() {
    let (mut gate, _jobs, _results) = gate();
    gate.ready.insert(key(2), launch(2));
    gate.event(message(3, "create"), 101).unwrap();
    let mut removed = false;
    gate.poll(101, |method, keys| {
        assert_eq!(method, "unsubscribeTokenTrade");
        assert_eq!(keys, [key(2)]);
        removed = true;
        Ok(())
    })
    .unwrap();
    assert!(removed);
    assert!(is_price(gate.event(message(2, "buy"), 102).unwrap()));
}
