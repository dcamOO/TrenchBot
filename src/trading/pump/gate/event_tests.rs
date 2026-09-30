use super::test_support::*;
use crate::trading::{
    Event,
    archive::model::{Deployment, Progress},
};

#[test]
fn queues_without_blocking_prices_and_bounds_pending_work() {
    let (mut gate, jobs, _results) = gate();
    assert!(gate.event(message(2, "create"), 100).unwrap().is_none());
    assert_eq!(jobs.try_recv().unwrap().mint, key(2));
    assert!(is_price(gate.event(message(3, "buy"), 101).unwrap()));
    assert!(gate.event(message(2, "create"), 101).unwrap().is_none());
    assert!(jobs.try_recv().is_err());
    for mint in 3..18 {
        gate.event(message(mint, "create"), 102).unwrap();
    }
    assert!(gate.event(message(18, "create"), 102).is_err());
    assert_eq!(gate.pending.len(), 16);
}

#[test]
fn cached_over_limit_wallet_never_enters_rpc_queue() {
    let (mut gate, jobs, _results) = gate();
    let tokens: Vec<_> = (2..6)
        .map(|n| Deployment {
            mint: key(n),
            creator: key(1),
            launched_at: 90,
        })
        .collect();
    gate.db
        .checkpoint(&key(1), &Progress::default(), &tokens)
        .unwrap();
    assert!(gate.event(message(6, "create"), 100).is_err());
    assert!(jobs.try_recv().is_err());
}

#[test]
fn subsequent_deploy_invalidates_checks_for_the_same_creator() {
    let (mut gate, _jobs, _results) = gate();
    gate.event(message(2, "create"), 100).unwrap();
    gate.ready.insert(key(3), launch(3));
    gate.event(message(4, "create"), 101).unwrap();
    assert!(gate.invalidated.contains(&key(2)));
    assert!(gate.invalidated.contains(&key(3)));
    assert!(!gate.invalidated.contains(&key(4)));
}

#[test]
fn verified_candidate_uses_new_trade_price_and_observation_time() {
    let (mut gate, _jobs, _results) = gate();
    gate.ready.insert(key(2), launch(2));
    let Some(Event::Launch(candidate)) = gate.event(message(2, "buy"), 150).unwrap() else {
        panic!()
    };
    assert_eq!(candidate.price_sol, 0.03);
    assert_eq!(candidate.observed_at, 150);
    assert_eq!(candidate.launched_at, 100);
    assert!(gate.ready.is_empty());
}
