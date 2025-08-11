use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    },
};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    pub receiver: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = mint_authority,
        associated_token::mint = mint_account,
        associated_token::authority = receiver,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintToken<'info> {
    pub fn mint_token(&self, amount: u64) -> Result<()> {
        msg!(
            "Minting tokens to associated token account, {}",
            self.associated_token_program.key()
        );
        msg!("Mint: {}", self.mint_account.key());

        let mint_to_cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                authority: self.mint_authority.to_account_info(),
                mint: self.mint_account.to_account_info(),
                to: self.associated_token_account.to_account_info(),
            },
        );

        mint_to(
            mint_to_cpi_ctx,
            amount * 10u64.pow(self.mint_account.decimals as u32),
        )?;

        msg!("Token minted successfully");
        Ok(())
    }
}
