use TrenchBot::trading::demo::run_demo;

#[test]
fn demo_completes_end_to_end_and_generates_pnl_file() {
    let result = run_demo();
    assert!(result.is_ok());
    assert!(std::path::Path::new("demo_pnl.csv").exists());
    let _ = std::fs::remove_file("demo_pnl.csv");
}
