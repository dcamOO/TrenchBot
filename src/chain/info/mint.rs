use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022_interface::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    state::Mint as TokenMint,
};

use super::super::ticker::Asset;

#[derive(Debug)]
pub(crate) struct MintInfo {
    pub decimals: u8,
    pub supply: u64,

    pub mint_authority: Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,

    pub token_program: Pubkey,

    pub extensions: Vec<String>,

    pub largest_accounts: Vec<LargestAccount>,
}

#[derive(Debug)]
pub(crate) struct LargestAccount {
    pub address: Pubkey,
    pub amount: u64,
    pub ui_amount: Option<f64>,
}

impl Asset {
    pub(crate) fn get_mint_info(&self) -> Result<MintInfo> {
        let account = self.rpc.get_account(&self.mint)?;

        let mint = TokenMint::unpack(&account.data)?;

        let extensions = self.get_extensions(
            &account.data,
            &account.owner,
        )?;

        let largest_accounts = self.get_largest_accounts()?;

        Ok(MintInfo {
            decimals: mint.decimals,
            supply: mint.supply,

            mint_authority: mint.mint_authority,
            freeze_authority: mint.freeze_authority,

            token_program: account.owner,

            extensions,
            largest_accounts,
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

    fn get_largest_accounts(
        &self,
    ) -> Result<Vec<LargestAccount>> {
        let accounts =
            self.rpc.get_token_largest_accounts(&self.mint)?;

        let accounts = accounts
            .into_iter()
            .map(|account| {
                Ok(LargestAccount {
                    address: account.address.parse()?,
                    amount: account.amount.amount.parse()?,
                    ui_amount: account.amount.ui_amount,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(accounts)
    }
}
