use super::common::*;

#[test]
fn scans_all_pages_and_only_refreshes_new_transactions() {
    let mut db = Store::open(":memory:").unwrap();
    let mut rpc = Mock::new(&[("c", Some(3)), ("b", None), ("a", Some(2))]);
    let report = scan(&mut db, &mut rpc, &key(1), 3).unwrap();
    assert!(report.complete && !report.over_limit);
    assert_eq!(report.tokens.len(), 2);
    assert_eq!(rpc.calls.len(), 6); // Three pages, three transactions, including the oldest.
    rpc.order.insert(0, "d".into());
    rpc.transactions
        .insert("d".into(), transaction("d", Some(4)));
    rpc.calls.clear();
    assert!(scan(&mut db, &mut rpc, &key(1), 3).unwrap().complete);
    assert_eq!(rpc.calls, ["page:None", "d"]);
}

#[test]
fn stops_at_x_plus_one_and_cached_rejection_spends_nothing() {
    let mut db = Store::open(":memory:").unwrap();
    let mut rpc = Mock::new(&[("c", Some(4)), ("b", Some(3)), ("a", Some(2))]);
    let report = scan(&mut db, &mut rpc, &key(1), 1).unwrap();
    assert!(report.over_limit && !report.complete);
    assert!(!rpc.calls.contains(&"a".into()));
    rpc.calls.clear();
    assert!(scan(&mut db, &mut rpc, &key(1), 1).unwrap().over_limit);
    assert!(rpc.calls.is_empty());
    // Raising X resumes the unfinished scan rather than trusting the partial cache.
    assert!(scan(&mut db, &mut rpc, &key(1), 3).unwrap().complete);
    assert_eq!(db.tokens(&key(1)).unwrap().len(), 3);
}

#[test]
fn missing_transaction_does_not_advance_and_resume_catches_up() {
    let mut db = Store::open(":memory:").unwrap();
    let mut rpc = Mock::new(&[("b", Some(3)), ("a", Some(2))]);
    rpc.transactions.remove("a");
    assert!(scan(&mut db, &mut rpc, &key(1), 5).is_err());
    assert_eq!(db.progress(&key(1)).unwrap().before.as_deref(), Some("b"));
    rpc.transactions
        .insert("a".into(), transaction("a", Some(2)));
    rpc.order.insert(0, "c".into());
    rpc.transactions
        .insert("c".into(), transaction("c", Some(4)));
    let report = scan(&mut db, &mut rpc, &key(1), 5).unwrap();
    assert!(report.complete);
    assert_eq!(report.tokens.len(), 3);
    assert_eq!(db.progress(&key(1)).unwrap().anchor.as_deref(), Some("c"));
}

#[test]
fn missing_incremental_anchor_cannot_prove_completeness() {
    let mut db = Store::open(":memory:").unwrap();
    let mut rpc = Mock::new(&[("a", Some(2))]);
    scan(&mut db, &mut rpc, &key(1), 5).unwrap();
    let mut pruned = Mock::new(&[("b", Some(3))]);
    assert!(scan(&mut db, &mut pruned, &key(1), 5).is_err());
    assert!(db.progress(&key(1)).unwrap().active);
}
