mod findings;
mod stats;

use serde::Serialize;
pub(crate) use findings::Finding;
pub(crate) use stats::DerivedStats;

use super::super::info::TokenInfo;

#[derive(Debug, Serialize)]
pub(crate) struct ProcessedInfo {
    pub token: TokenInfo,
    pub findings: Vec<Finding>,
}

pub(crate) fn process(token: TokenInfo) -> ProcessedInfo {
    let findings = findings::collect(&token);

    ProcessedInfo {
        token,
        findings
    }
}
