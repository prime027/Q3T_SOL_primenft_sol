use anchor_lang::prelude::*;
use anchor_spl:: {token::{close_account, CloseAccount}, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};


use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,
    maker_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    listing: Box<Account<'info, Listing>>,

    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = maker_mint,
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft (&mut self) -> Result<()> {
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let account = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), account, signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals);

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        let close_ctx = CpiContext::new_with_signer(cpi_program, close_accounts, signer_seeds);

        close_account(close_ctx)?;

        Ok(())
    }
}