use TrenchBot::trading::{
    Event,
    pump::{history::Histories, message::PumpMessage},
};
use std::collections::HashMap;

fn payload() -> serde_json::Value {
    serde_json::json!({"txType":"create", "mint":"mint", "traderPublicKey":"creator",
        "pool":"pump", "vSolInBondingCurve":30.0, "vTokensInBondingCurve":1000.0})
}

#[test]
fn accepts_pump_deploys_and_ignores_other_platforms_and_acknowledgements() {
    let p = payload();
    assert_eq!(
        PumpMessage::parse(&p.to_string())
            .unwrap()
            .unwrap()
            .price()
            .unwrap(),
        0.03
    );
    let mut p = p;
    p["pool"] = "bonk".into();
    assert!(PumpMessage::parse(&p.to_string()).unwrap().is_none());
    assert!(
        PumpMessage::parse(r#"{"message":"subscribed"}"#)
            .unwrap()
            .is_none()
    );
    p["pool"] = "pump".into();
    p["vTokensInBondingCurve"] = 0.into();
    assert!(PumpMessage::parse(&p.to_string()).is_err());
}

#[test]
fn observed_deploys_accumulate_without_claiming_a_complete_history() {
    let mut histories = Histories(HashMap::new());
    for now in [100, 200] {
        let message = PumpMessage::parse(&payload().to_string()).unwrap().unwrap();
        let Event::Launch(l) = histories.event(message, now).unwrap() else {
            panic!()
        };
        assert_eq!(l.creator, "creator");
        assert_eq!(l.launched_at, 100);
        assert_eq!(l.history.tokens.len(), 1);
        assert!(!l.history.complete);
        assert_eq!(l.platform, "pump_fun");
    }
    let mut p = payload();
    p["mint"] = "second".into();
    histories
        .event(PumpMessage::parse(&p.to_string()).unwrap().unwrap(), 300)
        .unwrap();
    assert_eq!(histories.0["creator"].tokens.len(), 2);
    histories.0.get_mut("creator").unwrap().complete = true;
    histories.invalidate();
    assert!(!histories.0["creator"].complete);
}

#[test]
fn trades_update_prices_without_treating_traders_as_creators() {
    let mut histories = Histories(HashMap::new());
    let mut p = payload();
    p["txType"] = "buy".into();
    p["traderPublicKey"] = "buyer-not-creator".into();
    let event = histories
        .event(PumpMessage::parse(&p.to_string()).unwrap().unwrap(), 100)
        .unwrap();
    assert!(matches!(
        event,
        Event::Price {
            price_sol: 0.03,
            ..
        }
    ));
    assert!(histories.0.is_empty());
}
