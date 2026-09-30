use super::Gate;
use crate::trading::pump::socket::Feed;
use anyhow::Result;

impl Gate {
    pub fn poll(&mut self, feed: &mut Feed, now: u64) -> Result<()> {
        while let Ok((mint, result)) = self.worker.results.try_recv() {
            self.pending.remove(&mint);
            if self.invalidated.remove(&mint) {
                continue;
            }
            match result {
                Ok(launch)
                    if launch.launched_at <= now
                        && now - launch.launched_at <= self.config.max_launch_age_seconds =>
                {
                    // Pay for trade data only after history and ATH checks have passed.
                    feed.subscribe("subscribeTokenTrade", std::slice::from_ref(&mint))?;
                    self.ready.insert(mint, launch);
                }
                Ok(_) => eprintln!("Candidato expirou durante verificação histórica: {mint}"),
                Err(error) => eprintln!("Candidato {mint} ignorado: {error}"),
            }
        }
        let expired: Vec<_> = self
            .ready
            .iter()
            .filter(|(_, launch)| {
                self.invalidated.contains(&launch.mint)
                    || now.saturating_sub(launch.launched_at) > self.config.max_launch_age_seconds
            })
            .map(|(mint, _)| mint.clone())
            .collect();
        for mint in expired {
            self.ready.remove(&mint);
            self.invalidated.remove(&mint);
            feed.subscribe("unsubscribeTokenTrade", &[mint])?;
        }
        Ok(())
    }
}
