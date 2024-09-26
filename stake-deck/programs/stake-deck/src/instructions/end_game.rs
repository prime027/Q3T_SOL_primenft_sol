use std::str::FromStr;

use anchor_lang::prelude::*;
use crate::{error::StakeDeckError, state::{GameState, GameAccount, Vault}};

#[derive(Accounts)]
pub struct EndGame<'info> {
    // #[account(mut)]
    // pub admin: Signer<'info>, // The admin ending the game
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>, // The game account
    #[account(mut, close = first_player)]
    pub vault: Account<'info, Vault>, // The vault account holding the funds
    #[account(mut)]
    pub winner: Signer<'info>,// The winner's account
    /// CHECK: Admin public key is fixed and trusted
    #[account(address = Pubkey::from_str("4Lv5tenh6jZVeNyBwhQsj39mSnY6g2s9u56kmZ8ZgtEj").unwrap())] // Admin (game creator) fixed public key
    pub admin: UncheckedAccount<'info>, // Admin's account (fixed creator)
    pub first_player: SystemAccount<'info>, // The first player who initialized the vault
    pub system_program: Program<'info, System>, // Required for system instructions
}

impl<'info> EndGame<'info> {
    pub fn end_game(ctx: Context<EndGame>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let vault = &mut ctx.accounts.vault;
        let winner = &mut ctx.accounts.winner;
        let admin = &mut ctx.accounts.admin;
        // let first_player = &mut ctx.accounts.first_player;

        require!(
            game_account.game_state == GameState::InProgress,
            StakeDeckError::GameNotInProgress
        );

         // Calculate the amounts to distribute
        let vault_balance = **vault.to_account_info().lamports.borrow();
        let fee_amount = vault_balance * game_account.fee_percentage as u64 / 100; // Fee based on fee_percentage
        let winner_amount = vault_balance - fee_amount; // Remaining balance goes to the winner

        // Transfer 95% of the vault's balance to the winner
        **vault.to_account_info().lamports.borrow_mut() -= winner_amount;
        **winner.to_account_info().lamports.borrow_mut() += winner_amount;

        // Transfer 5% of the vault's balance to the admin (fixed pubkey)
        **vault.to_account_info().lamports.borrow_mut() -= fee_amount;
        **admin.to_account_info().lamports.borrow_mut() += fee_amount;

        // Set the game state to Finished
        game_account.game_state = GameState::Completed;

        Ok(())
    }
}