use super::model::Report;
use crate::trading::{CreatorHistory, HistoricalToken, Launch};
use anyhow::{Context, Result, ensure};
use std::collections::{HashMap, HashSet};

/// ATH is supplied separately; enumeration and timestamps come only from archival data.
pub fn enrich(
    launch: &mut Launch,
    report: Report,
    ath: &HashMap<String, CreatorHistory>,
) -> Result<()> {
    ensure!(
        report.creator == launch.creator,
        "histórico de outro criador"
    );
    ensure!(
        !report.over_limit,
        "token e carteira descartados: total de deployments excede X"
    );
    ensure!(report.complete, "histórico incompleto");
    let candidate = report
        .tokens
        .iter()
        .find(|d| d.mint == launch.mint)
        .context("deployment candidato ainda não confirmado no histórico")?;
    launch.launched_at = candidate.launched_at;
    let source = ath.get(&launch.creator).context("ATH histórico ausente")?;
    ensure!(source.creator == launch.creator, "ATH de outro criador");
    let mut seen = HashSet::new();
    ensure!(
        source.tokens.iter().all(|t| seen.insert(&t.mint)),
        "ATH duplicado"
    );
    let tokens = report
        .tokens
        .into_iter()
        .map(|d| {
            let value = if d.mint == launch.mint || d.launched_at >= launch.launched_at {
                0.0 // Not used as evidence of a prior successful launch.
            } else {
                source
                    .tokens
                    .iter()
                    .find(|t| t.mint == d.mint)
                    .context("ATH ausente para deployment anterior")?
                    .ath_market_cap_usd
            };
            ensure!(value.is_finite() && value >= 0.0, "ATH inválido");
            Ok(HistoricalToken {
                mint: d.mint,
                launched_at: d.launched_at,
                ath_market_cap_usd: value,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    launch.history = CreatorHistory {
        creator: launch.creator.clone(),
        complete: true,
        tokens,
    };
    Ok(())
}
