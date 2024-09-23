use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::error::StakeDeckError;
use crate::GameAccount;
use crate::state::PlayersAccount;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"game", game_account.key().as_ref()],
        bump = game_account.bump,
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(
        mut,
        seeds = [b"players", game_account.key().as_ref(), user.key().as_ref()],
        bump = players_account.bump,
    )]
    pub players_account: Account<'info, PlayersAccount>,
    #[account(
        mut,
        seeds = [b"vault", game_account.key().as_ref()],
        bump
    )]
    pub vault_account: SystemAccount<'info>, // Vault to hold funds
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, bet_amount: u64) -> Result<()> {
        // Ensure bet_amount is non-zero
        require!(bet_amount > 0, StakeDeckError::InvalidBetAmount);
        // let game = &mut self.game_account;
        // let players = &mut self.players_account;

        require!(
            self.user.lamports() >= bet_amount,
            StakeDeckError::InsufficientFunds
        );

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, bet_amount)?;

        if self.players_account.player_1 == self.user.key() {
            self.game_account.player_1_bet += bet_amount;
        } else if self.players_account.player_2 == self.user.key() {
            self.game_account.player_2_bet += bet_amount;
        } else {
            return Err(StakeDeckError::InvalidPlayerAccount.into());
        }

        Ok(())
    }
}