use super::{
    cli::cache_path,
    deadline::Deadline,
    enrich::enrich,
    rpc::{Rpc, now},
    scan::scan,
    store::Store,
};
use crate::trading::{Launch, TradingConfig, pump::history::Histories};
use anyhow::{Result, ensure};
use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread,
    time::Duration,
};

pub struct Worker {
    pub jobs: SyncSender<Launch>,
    pub results: Receiver<(String, Result<Launch>)>,
}

impl Worker {
    pub fn start(config: TradingConfig, ath: Histories) -> Result<Self> {
        let path = cache_path();
        let mut db = Store::open(&path)?;
        let mut rpc = Rpc::from_env(&path)?;
        let (jobs, incoming) = mpsc::sync_channel::<Launch>(16);
        let (outgoing, results) = mpsc::sync_channel(16);
        thread::Builder::new()
            .name("creator-archive".into())
            .spawn(move || {
                while let Ok(mut launch) = incoming.recv() {
                    let mint = launch.mint.clone();
                    let result = (|| -> Result<Launch> {
                        let expires_at = launch
                            .observed_at
                            .saturating_add(config.max_launch_age_seconds);
                        loop {
                            ensure!(now()? <= expires_at, "candidato expirou na fila");
                            let mut source = Deadline {
                                source: &mut rpc,
                                expires_at,
                            };
                            let report = scan(
                                &mut db,
                                &mut source,
                                &launch.creator,
                                config.max_creator_launches,
                            )?;
                            ensure!(
                                !report.over_limit,
                                "token e carteira descartados: deployments excedem X"
                            );
                            if report.tokens.iter().any(|d| d.mint == launch.mint) {
                                enrich(&mut launch, report, &ath.0)?;
                                ensure!(
                                    launch.history.tokens.iter().any(|token| token.mint
                                        != launch.mint
                                        && token.launched_at < launch.launched_at
                                        && token.ath_market_cap_usd
                                            > config.min_ath_market_cap_usd),
                                    "criador sem lançamento anterior com ATH acima de Y"
                                );
                                return Ok(launch);
                            }
                            // The stream is processed; wait for finality before trusting a launch.
                            thread::sleep(Duration::from_secs(2));
                        }
                    })();
                    if outgoing.send((mint, result)).is_err() {
                        break;
                    }
                }
            })?;
        Ok(Self { jobs, results })
    }
}
