use super::store::Store;
use anyhow::{Result, ensure};

impl Store {
    /// Charge before sending, including failed attempts. Shared by processes using this DB.
    pub fn reserve_request(&mut self, now: u64, limit: u64) -> Result<()> {
        ensure!(limit > 0, "orçamento diário deve ser positivo");
        let tx = self
            .0
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let day = now / 86400;
        tx.execute("INSERT OR IGNORE INTO requests VALUES (?1,0)", [day])?;
        let changed = tx.execute(
            "UPDATE requests SET used=used+1 WHERE day=?1 AND used<?2",
            (day, limit),
        )?;
        ensure!(
            changed == 1,
            "orçamento diário RPC esgotado; histórico permanece incompleto"
        );
        tx.commit()?;
        Ok(())
    }
}
