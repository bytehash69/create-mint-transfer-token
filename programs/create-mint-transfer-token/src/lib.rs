use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

declare_id!("CBtD37sphvuSpcnFYasKK3gYxa4FK6hrNgXJPoAEYZ7M");

#[program]
pub mod create_mint_transfer_token {

    use super::*;

    pub fn create_token(
        ctx: Context<CreateMint>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        ctx.accounts
            .create_mint(token_name, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        ctx.accounts.mint_token(amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_token(amount)
    }
}
