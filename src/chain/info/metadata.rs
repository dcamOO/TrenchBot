use anyhow::Result;
use mpl_token_metadata::accounts::Metadata;
use solana_sdk::pubkey::Pubkey;

use super::super::ticker::Asset;

#[derive(Debug)]
pub(crate) struct MetadataInfo {
    pub name: String,
    pub symbol: String,
    pub uri: String,

    pub update_authority: String,
    pub is_mutable: bool,

    pub creators: Option<Vec<CreatorInfo>>,
}

#[derive(Debug)]
pub(crate) struct CreatorInfo {
    pub address: String,
    pub verified: bool,
    pub share: u8,
}

impl Asset {
    pub(crate) fn get_metadata(
        &self,
    ) -> Result<Option<MetadataInfo>> {
        let mint =
            solana_program::pubkey::Pubkey::new_from_array(
                self.mint.to_bytes(),
            );

        let (metadata_pda, _) =
            Metadata::find_pda(&mint);

        let metadata_pda =
            Pubkey::new_from_array(
                metadata_pda.to_bytes(),
            );

        let data = match self.rpc.get_account_data(&metadata_pda) {
            Ok(data) => data,
            Err(_) => return Ok(None),
        };

        let metadata =
            Metadata::safe_deserialize(&data)?;

        let creators = metadata.creators.map(|creators| {
            creators
                .into_iter()
                .map(|creator| CreatorInfo {
                    address: creator.address.to_string(),
                    verified: creator.verified,
                    share: creator.share,
                })
                .collect()
        });

        Ok(Some(MetadataInfo {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.uri,
            update_authority: metadata.update_authority.to_string(),
            is_mutable: metadata.is_mutable,
            creators,
        }))
    }
}
