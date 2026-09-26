use anyhow::Result;
use serde::Serialize;
use solana_program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022_interface::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    state::Mint as TokenMint,
};

use super::super::ticker::Asset;

#[derive(Debug, Serialize)]
pub(crate) struct MintInfo {
    pub decimals: u8,
    pub supply: u64,

    pub mint_authority: Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,

    pub token_program: Pubkey,

    pub extensions: Vec<String>,
}

impl Asset {
    pub(crate) fn get_mint_info(&self) -> Result<MintInfo> {
        let account = self.rpc.get_account(&self.mint)?;

        let mint = TokenMint::unpack(&account.data)?;

        let extensions = self.get_extensions(
            &account.data,
            &account.owner,
        )?;

        Ok(MintInfo {
            decimals: mint.decimals,
            supply: mint.supply,

            mint_authority: mint.mint_authority.into(),
            freeze_authority: mint.freeze_authority.into(),

            token_program: account.owner,

            extensions,
        })
    }

    fn get_extensions(
        &self,
        data: &[u8],
        token_program: &Pubkey,
    ) -> Result<Vec<String>> {
        if *token_program != spl_token_2022_interface::id() {
            return Ok(Vec::new());
        }

        let state = StateWithExtensions::<TokenMint>::unpack(data)?;

        let extensions = state
            .get_extension_types()?
            .into_iter()
            .map(|extension| format!("{extension:?}"))
            .collect();

        Ok(extensions)
    }
}
