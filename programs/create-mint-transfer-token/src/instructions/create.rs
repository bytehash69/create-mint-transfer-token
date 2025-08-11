use {
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
            Metadata,
        },
        token::{Mint, Token},
    }
};

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer, 
        mint::decimals = 9,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key()
    )]
    pub mint_account: Account<'info, Mint>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>
}


impl<'info> CreateMint<'info> {
    pub fn create_mint(
        &self,
        token_name: String,
        token_symbol: String,
        token_uri: String
    ) -> Result<()> {
        msg!("Creating metadata account");

        let create_metadata_cpi_ctx = CpiContext::new(self.token_metadata_program.to_account_info(), CreateMetadataAccountsV3 {
            metadata: self.metadata_account.to_account_info(),
            mint: self.mint_account.to_account_info(),
            mint_authority: self.payer.to_account_info(),
            payer: self.payer.to_account_info(),
            rent: self.rent.to_account_info(),
            system_program: self.system_program.to_account_info(),
            update_authority: self.payer.to_account_info()
        });

        create_metadata_accounts_v3(
            create_metadata_cpi_ctx, 
            DataV2 { name: token_name, symbol: token_symbol, uri: token_uri, seller_fee_basis_points: 0, collection: None, creators: None, uses: None },
            false,
            true,
            None
        )?;

        msg!("Token created successfully");

        Ok(())
    }
}