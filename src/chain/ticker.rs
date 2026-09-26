use std::str::FromStr;

use anyhow::{anyhow, Result};
use mpl_token_metadata::accounts::Metadata;
use reqwest::blocking::Client;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Deserialize)]
struct Token {
    id: String,
    name: String,
    symbol: String,
    decimals: u8,
}

struct Asset {
    ticker: String,
    rpc: RpcClient,
    mint: Pubkey,
}

impl Asset {
    fn new(ticker: &str, rpc_url: &str, jupiter_key: &str) -> Result<Self> {
        let rpc = Self::set_rpc_client(rpc_url);
        let mint = Self::find_mint(ticker, jupiter_key)?;

        Ok(Self {
            ticker: ticker.to_string(),
            rpc,
            mint,
        })
    }

    fn set_rpc_client(url: &str) -> RpcClient {
        RpcClient::new(url.to_string())
    }

    fn find_mint(ticker: &str, api_key: &str) -> Result<Pubkey> {
        let client = Client::new();

        let response = client
            .get("https://api.jup.ag/tokens/v2/search")
            .query(&[("query", ticker)])
            .header("x-api-key", api_key)
            .send()?
            .error_for_status()?
            .json::<Vec<Token>>()?;

        let token = response
            .into_iter()
            .find(|token| token.symbol.eq_ignore_ascii_case(ticker))
            .ok_or_else(|| anyhow!("Ticker não encontrado: {ticker}"))?;

        Ok(Pubkey::from_str(&token.id)?)
    }

    fn get_metadata(&self) -> Result<Metadata> {
        let metadata_program =
            Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;

        let (metadata_pda, _) = Pubkey::find_program_address(
            &[
                b"metadata",
                metadata_program.as_ref(),
                self.mint.as_ref(),
            ],
            &metadata_program,
        );

        let data = self.rpc.get_account_data(&metadata_pda)?;

        let metadata = Metadata::safe_deserialize(&data)?;

        Ok(metadata)
    }
}
