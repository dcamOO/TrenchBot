use super::model::{Deployment, Progress};
use anyhow::{Result, ensure};
use rusqlite::{Connection, OptionalExtension, params};
use std::{path::Path, time::Duration};

pub struct Store(pub(super) Connection);

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Connection::open(path)?;
        db.busy_timeout(Duration::from_secs(5))?;
        let version: u32 = db.pragma_query_value(None, "user_version", |r| r.get(0))?;
        ensure!(version <= 1, "versão do cache não suportada");
        db.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;")?;
        db.execute_batch(include_str!("schema.sql"))?;
        db.pragma_update(None, "user_version", 1)?;
        Ok(Self(db))
    }

    pub fn tokens(&self, wallet: &str) -> Result<Vec<Deployment>> {
        let mut query = self.0.prepare(
            "SELECT mint, creator, launched_at FROM deployments WHERE creator=?1 ORDER BY mint",
        )?;
        Ok(query
            .query_map([wallet], |r| {
                Ok(Deployment {
                    mint: r.get(0)?,
                    creator: r.get(1)?,
                    launched_at: r.get(2)?,
                })
            })?
            .collect::<Result<_, _>>()?)
    }

    pub fn progress(&self, wallet: &str) -> Result<Progress> {
        let json: Option<String> = self
            .0
            .query_row(
                "SELECT state FROM progress WHERE wallet=?1",
                [wallet],
                |r| r.get(0),
            )
            .optional()?;
        Ok(match json {
            Some(json) => serde_json::from_str(&json)?,
            None => Progress::default(),
        })
    }

    /// Deployments and cursor advance together, so a crash cannot skip a transaction.
    pub fn checkpoint(
        &mut self,
        wallet: &str,
        state: &Progress,
        tokens: &[Deployment],
    ) -> Result<()> {
        let tx = self.0.transaction()?;
        for token in tokens {
            let old: Option<(String, u64)> = tx
                .query_row(
                    "SELECT creator, launched_at FROM deployments WHERE mint=?1",
                    [&token.mint],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()?;
            ensure!(
                old.is_none_or(|old| old == (token.creator.clone(), token.launched_at)),
                "deployment conflitante; cache não alterado"
            );
            tx.execute(
                "INSERT OR IGNORE INTO deployments VALUES (?1,?2,?3)",
                params![token.mint, token.creator, token.launched_at],
            )?;
        }
        tx.execute("INSERT INTO progress VALUES (?1,?2) ON CONFLICT(wallet) DO UPDATE SET state=excluded.state",
            params![wallet, serde_json::to_string(state)?])?;
        tx.commit()?;
        Ok(())
    }
}
