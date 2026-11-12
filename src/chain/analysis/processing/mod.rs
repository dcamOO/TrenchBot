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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::info::{
        JupiterInfo,
        MetadataInfo,
        MintInfo,
    };
    use solana_sdk::pubkey::Pubkey;

    fn make_token_info(
        mint_authority: Option<Pubkey>,
        freeze_authority: Option<Pubkey>,
        metadata: Option<MetadataInfo>,
        extensions: Vec<String>,
    ) -> TokenInfo {
        TokenInfo {
            mint: Pubkey::default(),
            ticker: "TEST".into(),
            mint_info: MintInfo {
                decimals: 9,
                supply: 1_000_000,
                mint_authority,
                freeze_authority,
                token_program: Pubkey::default(),
                extensions,
            },
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

    #[test]
    fn process_preserves_token_info() {
        let token = make_token_info(
            None,
            None,
            Some(make_metadata(false)),
            vec![],
        );

        let processed = process(token);

        assert_eq!(processed.token.ticker, "TEST");
        assert_eq!(processed.token.mint, Pubkey::default());
        assert_eq!(processed.token.mint_info.decimals, 9);
        assert_eq!(processed.token.mint_info.supply, 1_000_000);
    }

    #[test]
    fn process_collects_findings() {
        let token = make_token_info(
            Some(Pubkey::new_unique()),
            None,
            Some(make_metadata(false)),
            vec![],
        );

        let processed = process(token);

        assert_eq!(processed.findings.len(), 1);
        assert_eq!(
            processed.findings[0].message,
            "Mint authority is configured."
        );
    }

    #[test]
    fn process_without_findings() {
        let token = make_token_info(
            None,
            None,
            Some(make_metadata(false)),
            vec![],
        );

        let processed = process(token);

        assert!(processed.findings.is_empty());
    }

    #[test]
    fn process_with_multiple_findings() {
        let token = make_token_info(
            Some(Pubkey::new_unique()),
            Some(Pubkey::new_unique()),
            None,
            vec![
                "PermanentDelegate".into(),
                "Pausable".into(),
            ],
        );

        let processed = process(token);

        assert_eq!(processed.findings.len(), 5);
    }
}
