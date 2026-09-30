use super::common::*;

#[test]
fn counts_outer_and_inner_creates_but_not_failed_transactions() {
    let mut tx = transaction("a", Some(2));
    tx["meta"]["innerInstructions"] = json!([{"instructions":[instruction(3,true)]}]);
    let found = deployments(&tx, "a").unwrap();
    assert_eq!(
        found.iter().map(|d| &d.mint).collect::<Vec<_>>(),
        [&key(2), &key(3)]
    );
    assert!(found.iter().all(|d| d.creator == key(1)));
    tx["meta"]["err"] = json!({"InstructionError":[0,"Custom"]});
    assert!(deployments(&tx, "a").unwrap().is_empty());
}

#[test]
fn rejects_missing_metadata_and_unknown_instructions() {
    assert!(deployments(&Value::Null, "a").is_err());
    let mut tx = transaction("a", Some(2));
    assert!(deployments(&tx, "wrong").is_err());
    tx["meta"]["innerInstructions"] = Value::Null;
    assert!(deployments(&tx, "a").is_err());
    tx["meta"]["innerInstructions"] = json!([]);
    tx["transaction"]["message"]["instructions"][0]["data"] =
        json!(bs58::encode([0; 8]).into_string());
    assert!(deployments(&tx, "a").is_err());
}

#[test]
fn anchor_event_cpi_is_not_an_additional_deployment() {
    let mut tx = transaction("a", Some(2));
    let event =
        json!({"programId":PUMP,"data":bs58::encode([228,69,165,46,81,203,154,29]).into_string()});
    tx["meta"]["innerInstructions"] = json!([{"instructions":[event]}]);
    assert_eq!(deployments(&tx, "a").unwrap().len(), 1);
}
