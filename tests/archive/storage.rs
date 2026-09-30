use super::common::*;

#[test]
fn duplicate_mints_do_not_inflate_count_and_conflicts_roll_back() {
    let mut db = Store::open(":memory:").unwrap();
    let token = Deployment {
        mint: key(2),
        creator: key(1),
        launched_at: 100,
    };
    db.checkpoint(
        &key(1),
        &Progress::default(),
        &[token.clone(), token.clone()],
    )
    .unwrap();
    assert_eq!(db.tokens(&key(1)).unwrap().len(), 1);
    let state = Progress {
        before: Some("bad".into()),
        active: true,
        ..Default::default()
    };
    let mut conflict = token.clone();
    conflict.creator = key(8);
    assert!(db.checkpoint(&key(1), &state, &[conflict]).is_err());
    assert!(db.progress(&key(1)).unwrap().before.is_none());
    assert_eq!(db.tokens(&key(1)).unwrap(), [token]);
}

#[test]
fn daily_budget_and_progress_survive_reopening() {
    let path = std::env::temp_dir().join(format!(
        "trenchbot-cache-test-{}.sqlite",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    {
        let mut db = Store::open(&path).unwrap();
        db.reserve_request(100, 1).unwrap();
        assert!(db.reserve_request(200, 1).is_err());
        let state = Progress {
            before: Some("cursor".into()),
            active: true,
            ..Default::default()
        };
        db.checkpoint(&key(1), &state, &[]).unwrap();
    }
    {
        let mut db = Store::open(&path).unwrap();
        assert!(db.reserve_request(300, 1).is_err());
        db.reserve_request(86400, 1).unwrap();
        assert_eq!(
            db.progress(&key(1)).unwrap().before.as_deref(),
            Some("cursor")
        );
    }
    std::fs::remove_file(path).unwrap();
}
