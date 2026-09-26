use super::super::super::info::{
    MetadataInfo,
    MintInfo,
    TokenInfo,
};

use super::DerivedStats;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Finding {
    pub category: FindingCategory,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub(crate) enum FindingCategory {
    Authority,
    Metadata,
    Token2022,
}

pub(crate) fn collect(
    info: &TokenInfo,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    authorities(&info.mint_info, &mut findings);
    metadata(info.metadata.as_ref(), &mut findings);
    token_2022(&info.mint_info, &mut findings);

    findings
}

fn authorities(
    mint: &MintInfo,
    findings: &mut Vec<Finding>,
) {
    if mint.mint_authority.is_some() {
        findings.push(Finding {
            category: FindingCategory::Authority,
            message: "Mint authority is configured.".into(),
        });
    }

    if mint.freeze_authority.is_some() {
        findings.push(Finding {
            category: FindingCategory::Authority,
            message: "Freeze authority is configured.".into(),
        });
    }
}

fn metadata(
    metadata: Option<&MetadataInfo>,
    findings: &mut Vec<Finding>,
) {
    let Some(metadata) = metadata else {
        findings.push(Finding {
            category: FindingCategory::Metadata,
            message: "No Metaplex metadata account was found.".into(),
        });

        return;
    };

    if metadata.is_mutable {
        findings.push(Finding {
            category: FindingCategory::Metadata,
            message: "Metadata is mutable.".into(),
        });
    }
}

fn token_2022(
    mint: &MintInfo,
    findings: &mut Vec<Finding>,
) {
    for extension in &mint.extensions {
        let message = match extension.as_str() {
            "PermanentDelegate" =>
                Some("PermanentDelegate extension is enabled."),

            "TransferHook" =>
                Some("TransferHook extension is enabled."),

            "TransferFeeConfig" =>
                Some("TransferFeeConfig extension is enabled."),

            "Pausable" =>
                Some("Pausable extension is enabled."),

            _ => None,
        };

        if let Some(message) = message {
            findings.push(Finding {
                category: FindingCategory::Token2022,
                message: message.into(),
            });
        }
    }
}
