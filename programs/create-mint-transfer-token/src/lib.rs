use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

declare_id!("CBtD37sphvuSpcnFYasKK3gYxa4FK6hrNgXJPoAEYZ7M");

#[program]
pub mod create_mint_transfer_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
