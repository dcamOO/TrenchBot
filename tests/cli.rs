//! Integração: executa o binário real (args → config → replay → engine → broker → PnL → arquivo).
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const CONFIG: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/trading.json");
const EVENTS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/events.jsonl");

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trenchbot-cli-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn trenchbot(args: &[&str], cwd: &Path, balance: Option<&str>) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_TrenchBot"));
    cmd.args(args)
        .current_dir(cwd)
        .env_remove("TELEGRAM_BOT_TOKEN")
        .env_remove("TELEGRAM_CHAT_ID")
        .env_remove("PAPER_BALANCE_LAMPORTS");
    if let Some(balance) = balance {
        cmd.env("PAPER_BALANCE_LAMPORTS", balance);
    }
    cmd.output().unwrap()
}

fn export(name: &str, balance: Option<&str>) -> (Output, String) {
    let dir = temp_dir(name);
    let csv = dir.join("pnl.csv");
    let out = trenchbot(
        &["--export-pnl", CONFIG, EVENTS, csv.to_str().unwrap()],
        &dir,
        balance,
    );
    let content = std::fs::read_to_string(&csv).unwrap_or_default();
    let _ = std::fs::remove_dir_all(dir);
    (out, content)
}

fn actions(out: &Output) -> Vec<String> {
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .map(|v| {
            format!(
                "{}:{}",
                v["action"].as_str().unwrap(),
                v["reason"].as_str().unwrap_or("")
            )
        })
        .collect()
}

#[test]
fn export_pnl_replays_events_and_writes_report() {
    let (out, csv) = export("ok", None);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        actions(&out),
        [
            "bought:",
            "skipped:limites não atingidos",
            "sold:take_profit",
            "bought:",
            "sold:stop_loss"
        ]
    );
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(
        lines[1],
        "DEMO-A,1010,1030,100000000,150000000,50000000,0.050000,50.00,take_profit"
    );
    assert_eq!(
        lines[2],
        "DEMO-B,1050,1060,100000000,80000000,-20000000,-0.020000,-20.00,stop_loss"
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("(2 registros)"));
}

#[test]
fn export_pnl_with_insufficient_balance_reports_errors_and_empty_report() {
    let (out, csv) = export("poor", Some("1000"));
    assert!(out.status.success());
    assert!(
        actions(&out)
            .iter()
            .filter(|a| a.starts_with("error:"))
            .count()
            >= 2
    );
    assert_eq!(csv.lines().count(), 1);
}

#[test]
fn export_pnl_rejects_invalid_balance_variable() {
    let (out, csv) = export("nan", Some("muito"));
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("PAPER_BALANCE_LAMPORTS"));
    assert!(csv.is_empty());
}

#[test]
fn export_pnl_requires_output_path() {
    let dir = temp_dir("usage");
    let out = trenchbot(&["--export-pnl", CONFIG, EVENTS], &dir, None);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("uso: TrenchBot --export-pnl"));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn export_pnl_rejects_invalid_config() {
    let dir = temp_dir("config");
    let config = dir.join("bad.json");
    std::fs::write(&config, r#"{"stop_loss_percent": 0}"#).unwrap();
    let out = trenchbot(
        &["--export-pnl", config.to_str().unwrap(), EVENTS, "x.csv"],
        &dir,
        None,
    );
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("configuração de trading inválida"));
    assert!(!dir.join("x.csv").exists());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn demo_flag_writes_report_in_working_directory() {
    let dir = temp_dir("demo");
    let out = trenchbot(&["--demo"], &dir, None);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let csv = std::fs::read_to_string(dir.join("demo_pnl.csv")).unwrap();
    assert!(csv.lines().nth(1).unwrap().ends_with(",50.00,take_profit"));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn paper_flag_replays_events_without_report() {
    let dir = temp_dir("paper");
    let out = trenchbot(&["--paper", CONFIG, EVENTS], &dir, None);
    assert!(out.status.success());
    assert_eq!(actions(&out).len(), 5);
    assert!(String::from_utf8_lossy(&out.stderr).contains("nenhuma transação real"));
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn pump_paper_requires_api_key_before_connecting() {
    let dir = temp_dir("pump");
    let histories = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/histories.json");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_TrenchBot"));
    let out = cmd
        .args(["--pump-paper", CONFIG, histories])
        .current_dir(&dir)
        .env_remove("PUMPPORTAL_API_KEY")
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("PUMPPORTAL_API_KEY ausente"));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn unknown_flag_prints_usage() {
    let dir = temp_dir("unknown");
    let out = trenchbot(&["--voar"], &dir, None);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("uso: TrenchBot --paper"));
    let _ = std::fs::remove_dir_all(dir);
}
