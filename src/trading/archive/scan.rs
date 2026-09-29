use super::{
    decode,
    model::{Archive, Report},
    store::Store,
};
use anyhow::{Result, ensure};
use std::collections::HashSet;

pub fn scan(db: &mut Store, rpc: &mut impl Archive, wallet: &str, max: usize) -> Result<Report> {
    ensure!(max > 0, "X deve ser positivo");
    let resumed = db.progress(wallet)?.active;
    let mut report = pass(db, rpc, wallet, max)?;
    // A resumed snapshot can be old: catch up before approving a new candidate.
    if resumed && !report.over_limit {
        report = pass(db, rpc, wallet, max)?;
    }
    Ok(report)
}

fn report(db: &Store, wallet: &str, max: usize, complete: bool) -> Result<Report> {
    let tokens = db.tokens(wallet)?;
    Ok(Report {
        creator: wallet.into(),
        complete,
        over_limit: tokens.len() > max,
        tokens,
    })
}

fn pass(db: &mut Store, rpc: &mut impl Archive, wallet: &str, max: usize) -> Result<Report> {
    if db.tokens(wallet)?.len() > max {
        return report(db, wallet, max, false);
    }
    let mut state = db.progress(wallet)?;
    if !state.active {
        state.active = true;
        state.head = None;
        state.before = None;
        db.checkpoint(wallet, &state, &[])?;
    }
    let mut visited = HashSet::new();
    if let Some(before) = &state.before {
        visited.insert(before.clone());
    }
    loop {
        let page = rpc.signatures(wallet, state.before.as_deref())?;
        let mut finished = page.is_empty();
        if finished {
            ensure!(
                state.anchor.is_none(),
                "histórico não alcançou a âncora anterior"
            );
        }
        for signature in page {
            if state.anchor.as_ref() == Some(&signature) {
                finished = true;
                break;
            }
            ensure!(visited.insert(signature.clone()), "paginação não avançou");
            let tx = rpc.transaction(&signature)?;
            let tokens = decode::deployments(&tx, &signature)?;
            if state.head.is_none() {
                state.head = Some(signature.clone());
            }
            state.before = Some(signature);
            db.checkpoint(wallet, &state, &tokens)?;
            if db.tokens(wallet)?.len() > max {
                return report(db, wallet, max, false);
            }
        }
        if finished {
            state.anchor = state.head.take().or(state.anchor);
            state.before = None;
            state.active = false;
            db.checkpoint(wallet, &state, &[])?;
            return report(db, wallet, max, true);
        }
    }
}
