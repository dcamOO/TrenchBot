use super::common::*;
use TrenchBot::trading::{
    CreatorHistory, HistoricalToken, Launch,
    archive::{enrich::enrich, model::Report},
};
use std::collections::HashMap;

fn fixture() -> (Launch, Report, HashMap<String, CreatorHistory>) {
    let history = CreatorHistory {
        creator: key(1),
        complete: false,
        tokens: vec![HistoricalToken {
            mint: key(2),
            launched_at: 999,
            ath_market_cap_usd: 2_000_000.0,
        }],
    };
    let launch = Launch {
        platform: "pump_fun".into(),
        mint: key(3),
        creator: key(1),
        launched_at: 900,
        observed_at: 900,
        price_sol: 1.0,
        history: history.clone(),
    };
    let report = Report {
        creator: key(1),
        complete: true,
        over_limit: false,
        tokens: vec![
            Deployment {
                mint: key(2),
                creator: key(1),
                launched_at: 100,
            },
            Deployment {
                mint: key(3),
                creator: key(1),
                launched_at: 200,
            },
        ],
    };
    (launch, report, HashMap::from([(key(1), history)]))
}

#[test]
fn uses_archival_timestamps_and_count_not_snapshot_completeness() {
    let (mut launch, report, ath) = fixture();
    enrich(&mut launch, report, &ath).unwrap();
    assert_eq!(launch.launched_at, 200);
    assert!(launch.history.complete);
    assert_eq!(launch.history.tokens.len(), 2);
    assert_eq!(launch.history.tokens[0].launched_at, 100);
    assert_eq!(launch.history.tokens[0].ath_market_cap_usd, 2_000_000.0);
}

#[test]
fn refuses_missing_ath_and_missing_candidate_or_incomplete_enumeration() {
    let (mut launch, report, _) = fixture();
    assert!(enrich(&mut launch, report, &HashMap::new()).is_err());
    let (mut launch, mut report, ath) = fixture();
    report.tokens.pop();
    assert!(enrich(&mut launch, report, &ath).is_err());
    let (mut launch, mut report, ath) = fixture();
    report.complete = false;
    assert!(enrich(&mut launch, report, &ath).is_err());
    let (mut launch, mut report, ath) = fixture();
    report.over_limit = true;
    assert!(enrich(&mut launch, report, &ath).is_err());
}
