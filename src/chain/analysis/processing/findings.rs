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


#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use crate::chain::info::JupiterInfo;

    fn make_mint_info(
        mint_authority: Option<Pubkey>,
        freeze_authority: Option<Pubkey>,
        extensions: Vec<String>,
    ) -> MintInfo {
        MintInfo {
            decimals: 9,
            supply: 1_000_000,
            mint_authority,
            freeze_authority,
            token_program: Pubkey::default(),
            extensions,
        }
    }

    fn make_token_info(
        mint_info: MintInfo,
        metadata: Option<MetadataInfo>,
    ) -> TokenInfo {
        TokenInfo {
            mint: Pubkey::default(),
            ticker: "TEST".into(),
            mint_info,
            metadata,
            jupiter: JupiterInfo {
                price_usd: None,
                liquidity: None,
                market_cap_usd: None,
                holder_count: None,
                verified: None,
                audit: None,
                first_pool: None,
                stats_5m: None,
                stats_1h: None,
                stats_6h: None,
                stats_24h: None,
            },
        }
    }

    fn make_metadata(is_mutable: bool) -> MetadataInfo {
        MetadataInfo {
            name: "Test Token".into(),
            symbol: "TEST".into(),
            uri: String::new(),
            update_authority: String::new(),
            is_mutable,
            creators: None,
        }
    }

    fn count_category(
        findings: &[Finding],
        category: &FindingCategory,
    ) -> usize {
        findings
            .iter()
            .filter(|finding| {
                matches!(
                    (&finding.category, category),
                    (FindingCategory::Authority, FindingCategory::Authority)
                    | (FindingCategory::Metadata, FindingCategory::Metadata)
                    | (FindingCategory::Token2022, FindingCategory::Token2022)
                )
            })
            .count()
    }

    #[test]
    fn no_authorities() {
        let mint = make_mint_info(None, None, vec![]);
        let info = make_token_info(mint, Some(make_metadata(false)));

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Authority),
            0
        );
    }

    #[test]
    fn mint_authority_configured() {
        let mint = make_mint_info(
            Some(Pubkey::new_unique()),
            None,
            vec![],
        );

        let info = make_token_info(mint, Some(make_metadata(false)));

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Authority),
            1
        );

        assert_eq!(
            findings[0].message,
            "Mint authority is configured."
        );
    }

    #[test]
    fn freeze_authority_configured() {
        let mint = make_mint_info(
            None,
            Some(Pubkey::new_unique()),
            vec![],
        );

        let info = make_token_info(mint, Some(make_metadata(false)));

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Authority),
            1
        );

        assert_eq!(
            findings[0].message,
            "Freeze authority is configured."
        );
    }

    #[test]
    fn metadata_missing() {
        let mint = make_mint_info(None, None, vec![]);
        let info = make_token_info(mint, None);

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Metadata),
            1
        );

        assert_eq!(
            findings[0].message,
            "No Metaplex metadata account was found."
        );
    }

    #[test]
    fn metadata_mutable() {
        let mint = make_mint_info(None, None, vec![]);
        let info = make_token_info(mint, Some(make_metadata(true)));

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Metadata),
            1
        );

        assert_eq!(
            findings[0].message,
            "Metadata is mutable."
        );
    }

    #[test]
    fn metadata_immutable() {
        let mint = make_mint_info(None, None, vec![]);
        let info = make_token_info(mint, Some(make_metadata(false)));

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Metadata),
            0
        );
    }

    #[test]
    fn known_extensions() {
        let extensions = vec![
            "PermanentDelegate".into(),
            "TransferHook".into(),
            "TransferFeeConfig".into(),
            "Pausable".into(),
        ];

        let mint = make_mint_info(None, None, extensions);
        let info = make_token_info(mint, Some(make_metadata(false)));

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Token2022),
            4
        );
    }

    #[test]
    fn unknown_extension() {
        let mint = make_mint_info(
            None,
            None,
            vec!["UnknownExtension".into()],
        );

        let info = make_token_info(mint, Some(make_metadata(false)));

        let findings = collect(&info);

        assert_eq!(
            count_category(&findings, &FindingCategory::Token2022),
            0
        );
    }

    #[test]
    fn multiple_findings() {
        let mint = make_mint_info(
            Some(Pubkey::new_unique()),
            Some(Pubkey::new_unique()),
            vec![
                "PermanentDelegate".into(),
                "Pausable".into(),
            ],
        );

        let info = make_token_info(mint, None);

        let findings = collect(&info);

        assert_eq!(findings.len(), 5);

        assert_eq!(
            count_category(&findings, &FindingCategory::Authority),
            2
        );

        assert_eq!(
            count_category(&findings, &FindingCategory::Metadata),
            1
        );

        assert_eq!(
            count_category(&findings, &FindingCategory::Token2022),
            2
        );
    }
}
