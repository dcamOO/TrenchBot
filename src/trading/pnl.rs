use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PnlRecord {
    pub mint: String,
    pub opened_at: u64,
    pub closed_at: u64,
    pub cost_lamports: u64,
    pub received_lamports: u64,
    pub pnl_lamports: i64,
    pub pnl_sol: f64,
    pub pnl_percent: f64,
    pub reason: String,
}

impl PnlRecord {
    pub fn new(
        mint: String,
        opened_at: u64,
        closed_at: u64,
        cost_lamports: u64,
        received_lamports: u64,
        reason: String,
    ) -> Self {
        let pnl_lamports = received_lamports as i64 - cost_lamports as i64;
        let pnl_sol = pnl_lamports as f64 / 1_000_000_000.0;
        let pnl_percent = if cost_lamports > 0 {
            (pnl_lamports as f64 / cost_lamports as f64) * 100.0
        } else {
            0.0
        };

        Self {
            mint,
            opened_at,
            closed_at,
            cost_lamports,
            received_lamports,
            pnl_lamports,
            pnl_sol,
            pnl_percent,
            reason,
        }
    }

    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{:.6},{:.2},{}",
            self.mint,
            self.opened_at,
            self.closed_at,
            self.cost_lamports,
            self.received_lamports,
            self.pnl_lamports,
            self.pnl_sol,
            self.pnl_percent,
            self.reason
        )
    }
}

pub fn export_csv(records: &[PnlRecord]) -> String {
    let mut csv = String::from("mint,opened_at,closed_at,cost_lamports,received_lamports,pnl_lamports,pnl_sol,pnl_percent,reason\n");
    for record in records {
        csv.push_str(&record.to_csv_row());
        csv.push('\n');
    }
    csv
}
