
use anyhow::Result;
use mpl_token_metadata::accounts::Metadata;
use serde::Serialize;
use solana_pubkey::Pubkey as MetadataPubkey;
use solana_sdk::pubkey::Pubkey;

use super::super::ticker::Asset;

#[derive(Debug, Serialize)]
pub(crate) struct MetadataInfo {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub update_authority: String,
    pub is_mutable: bool,
    pub creators: Option<Vec<CreatorInfo>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreatorInfo {
    pub address: String,
    pub verified: bool,
    pub share: u8,
}

impl Asset {
    pub(crate) fn get_metadata(&self) -> Result<Option<MetadataInfo>> {
        let mint = MetadataPubkey::new_from_array(self.mint.to_bytes());

        let (metadata_pda, _) = Metadata::find_pda(&mint);

        let metadata_pda = Pubkey::new_from_array(metadata_pda.to_bytes());

        let data = match self.rpc.get_account_data(&metadata_pda) {
            Ok(data) => data,
            Err(_) => return Ok(None),
        };

        let metadata = Metadata::safe_deserialize(&data)?;

        Ok(Some(process_metadata(metadata)))
    }
}

fn process_metadata(metadata: Metadata) -> MetadataInfo {
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

    MetadataInfo {
        name: metadata.name.trim_matches('\0').trim().to_string(),
        symbol: metadata.symbol.trim_matches('\0').trim().to_string(),
        uri: metadata.uri.trim_matches('\0').trim().to_string(),
        update_authority: metadata.update_authority.to_string(),
        is_mutable: metadata.is_mutable,
        creators,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpl_token_metadata::types::{Creator, Key};

    fn make_metadata(creators: Option<Vec<Creator>>) -> Metadata {
        Metadata {
            key: Key::MetadataV1,
            update_authority: MetadataPubkey::new_unique(),
            mint: MetadataPubkey::new_unique(),
            name: "Test Token\0".to_string(),
            symbol: "TEST\0".to_string(),
            uri: "https://example.com/metadata.json\0".to_string(),
            seller_fee_basis_points: 500,
            creators,
            primary_sale_happened: false,
            is_mutable: true,
            edition_nonce: None,
            token_standard: None,
            collection: None,
            uses: None,
            collection_details: None,
            programmable_config: None,
        }
    }

    fn make_creator(verified: bool, share: u8) -> Creator {
        Creator {
            address: MetadataPubkey::new_unique(),
            verified,
            share,
        }
    }

    #[test]
    fn process_metadata_preserves_fields() {
        let metadata = make_metadata(None);
        let info = process_metadata(metadata);

        assert_eq!(info.name, "Test Token");
        assert_eq!(info.symbol, "TEST");
        assert_eq!(info.uri, "https://example.com/metadata.json");
        assert!(info.is_mutable);
        assert!(info.creators.is_none());
    }

    #[test]
    fn process_metadata_removes_null_terminators() {
        let mut metadata = make_metadata(None);

        metadata.name = "\0Token Name\0".to_string();
        metadata.symbol = "\0TKN\0".to_string();
        metadata.uri = "\0https://example.com\0".to_string();

        let info = process_metadata(metadata);

        assert_eq!(info.name, "Token Name");
        assert_eq!(info.symbol, "TKN");
        assert_eq!(info.uri, "https://example.com");
    }

    #[test]
    fn process_metadata_trims_whitespace() {
        let mut metadata = make_metadata(None);

        metadata.name = "  Token Name  ".to_string();
        metadata.symbol = " TKN ".to_string();

        let info = process_metadata(metadata);

        assert_eq!(info.name, "Token Name");
        assert_eq!(info.symbol, "TKN");
    }

    #[test]
    fn process_metadata_without_creators() {
        let info = process_metadata(make_metadata(None));

        assert!(info.creators.is_none());
    }

    #[test]
    fn process_metadata_with_creators() {
        let creators = vec![
            make_creator(true, 70),
            make_creator(false, 30),
        ];

        let expected_addresses: Vec<String> = creators
            .iter()
            .map(|creator| creator.address.to_string())
            .collect();

        let info = process_metadata(make_metadata(Some(creators)));
        let processed = info.creators.unwrap();

        assert_eq!(processed.len(), 2);

        assert_eq!(processed[0].address, expected_addresses[0]);
        assert!(processed[0].verified);
        assert_eq!(processed[0].share, 70);

        assert_eq!(processed[1].address, expected_addresses[1]);
        assert!(!processed[1].verified);
        assert_eq!(processed[1].share, 30);
    }

    #[test]
    fn process_metadata_with_empty_creators() {
        let info = process_metadata(make_metadata(Some(vec![])));

        assert!(info.creators.unwrap().is_empty());
    }
}
