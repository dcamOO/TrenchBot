use TrenchBot::trading::demo::run_demo_to;

#[test]
fn demo_completes_end_to_end_and_generates_pnl_file() {
    let path = std::env::temp_dir().join(format!("trenchbot-demo-{}.csv", std::process::id()));
    run_demo_to(path.to_str().unwrap()).unwrap();
    let csv = std::fs::read_to_string(&path).unwrap();
    let _ = std::fs::remove_file(&path);
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[1].starts_with("DEMO-ROCKET-SOL-PUMP,1005,1025,100000000,"));
    assert!(lines[1].ends_with(",50.00,take_profit"));
}

#[test]
fn demo_fails_when_output_path_is_unwritable() {
    assert!(run_demo_to("/nonexistent-dir/demo_pnl.csv").is_err());
}
