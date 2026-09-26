mod findings;
mod stats;

pub(crate) use findings::Finding;
pub(crate) use stats::DerivedStats;

use super::super::info::TokenInfo;

#[derive(Debug)]
pub(crate) struct ProcessedInfo {
    pub token: TokenInfo,
    pub derived: DerivedStats,
    pub findings: Vec<Finding>,
}

pub(crate) fn process(token: TokenInfo) -> ProcessedInfo {
    let derived = stats::calculate(&token);
    let findings = findings::collect(&token, &derived);

    ProcessedInfo {
        token,
        derived,
        findings,
    }
}
