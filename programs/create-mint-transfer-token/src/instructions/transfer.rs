use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    pub receiver: SystemAccount<'info>,

    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = sender
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = receiver,
    )]
    pub receiver_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TransferToken<'info> {
    pub fn transfer_token(&self, amount: u64) -> Result<()> {
        msg!("Transferring tokens...");
        msg!("Mint: {}", self.mint_account.to_account_info().key());
        msg!("From Token Address: {}", self.sender_token_account.key());
        msg!("To Token Address: {}", self.receiver_token_account.key());

        let transfer_cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                authority: self.sender.to_account_info(),
                from: self.sender_token_account.to_account_info(),
                to: self.receiver_token_account.to_account_info(),
            },
        );

        transfer(transfer_cpi_ctx, amount);
        Ok(())
    }
}
