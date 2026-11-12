
use crate::chain::{
    analysis::processing::process,
    info::{JupiterInfo, MetadataInfo, MintInfo, TokenInfo},
};

use serde_json::json;
use solana_sdk::pubkey::Pubkey;

#[test]
fn token_analysis_pipeline() {
    // 1. Simula a resposta recebida da Jupiter.
    let jupiter_json = json!({
        "usdPrice": 0.000025,
        "liquidity": 500000.0,
        "mcap": 2000000.0,
        "holderCount": 1200,
        "isVerified": true,
        "audit": {
            "mintAuthorityDisabled": false,
            "freezeAuthorityDisabled": false,
            "topHoldersPercentage": 35.5
        },
        "stats5m": {
            "priceChange": 1.5,
            "buyVolume": 1000.0,
            "sellVolume": 500.0,
            "numBuys": 10,
            "numSells": 5,
            "numTraders": 12
        }
    });

    let jupiter: JupiterInfo =
        serde_json::from_value(jupiter_json).unwrap();

    // 2. Simula os dados coletados da Mint.
    let mint = MintInfo {
        decimals: 9,
        supply: 1_000_000,
        mint_authority: Some(Pubkey::new_unique()),
        freeze_authority: Some(Pubkey::new_unique()),
        token_program: Pubkey::default(),
        extensions: vec![
            "PermanentDelegate".into(),
            "Pausable".into(),
        ],
    };

    // 3. Simula os metadados processados da Metaplex.
    let metadata = MetadataInfo {
        name: "Test Token".into(),
        symbol: "TEST".into(),
        uri: "https://example.com/metadata.json".into(),
        update_authority: Pubkey::new_unique().to_string(),
        is_mutable: true,
        creators: None,
    };

    // 4. Reúne os dados na estrutura utilizada pelo processamento.
    let token = TokenInfo {
        mint: Pubkey::new_unique(),
        ticker: "TEST".into(),
        mint_info: mint,
        metadata: Some(metadata),
        jupiter,
    };

    // 5. Executa o processamento real do projeto.
    let processed = process(token);

    // 6. Verifica se os dados originais foram preservados.
    assert_eq!(processed.token.ticker, "TEST");
    assert_eq!(processed.token.mint_info.decimals, 9);
    assert_eq!(processed.token.mint_info.supply, 1_000_000);

    assert_eq!(
        processed.token.jupiter.price_usd,
        Some(0.000025)
    );

    assert_eq!(
        processed.token.jupiter.liquidity,
        Some(500000.0)
    );

    assert_eq!(
        processed.token.jupiter.stats_5m
            .as_ref()
            .unwrap()
            .buy_volume,
        Some(1000.0)
    );

    // 7. Verifica os findings produzidos pela análise.
    assert_eq!(processed.findings.len(), 5);

    let messages: Vec<&str> = processed
        .findings
        .iter()
        .map(|finding| finding.message.as_str())
        .collect();

    assert!(messages.contains(&"Mint authority is configured."));
    assert!(messages.contains(&"Freeze authority is configured."));
    assert!(messages.contains(&"Metadata is mutable."));

    assert!(messages.contains(
        &"PermanentDelegate extension is enabled."
    ));

    assert!(messages.contains(
        &"Pausable extension is enabled."
    ));

    // 8. Verifica se o resultado pode ser serializado para JSON.
    let output = serde_json::to_value(&processed).unwrap();

    assert_eq!(output["token"]["ticker"], "TEST");
    assert_eq!(output["findings"].as_array().unwrap().len(), 5);
}
