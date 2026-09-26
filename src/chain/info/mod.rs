mod jupiter;
mod metadata;
mod mint;

pub use jupiter::JupiterInfo;
pub use metadata::MetadataInfo;
pub use mint::MintInfo;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use super::ticker::Asset;

#[derive(Debug)]
pub struct TokenInfo {
    pub mint: Pubkey,
    pub ticker: String,

    pub mint_info: MintInfo,
    pub metadata: Option<MetadataInfo>,
    pub jupiter: JupiterInfo,
}

impl Asset {
    pub(crate) fn get_info(&self, jupiter_key: &str) -> Result<TokenInfo> {
        let mint_info = self.get_mint_info()?;
        let metadata = self.get_metadata()?;
        let jupiter = self.get_jupiter_info(jupiter_key)?;

        Ok(TokenInfo {
            mint: self.mint,
            ticker: self.ticker.clone(),
            mint_info,
            metadata,
            jupiter,
        })
    }
}
