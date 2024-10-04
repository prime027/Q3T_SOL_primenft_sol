use anchor_lang::prelude::*;
use solana_program::program::invoke;
use solana_program::system_instruction;

use crate::error::StakeDeckError;
use crate::{GameAccount, GameState, VaultState};

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // The player placing the bet
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>, // The game account
    // #[account(
    //     mut,
    //     seeds = [b"vault", game_account.key().as_ref()], // Ensure vault is seeded with game_account
    //     bump
    // )]
    // pub vault: Account<'info, Vault>, // The vault account to hold the SOL
    // #[account(
    //     seeds = [b"vault", vault_state.key().as_ref()],
    //     bump = vault_state.vault_bump,
    // )]
    // pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", game_account.key().as_ref()],  // Ensure vault is seeded with game_account
        bump,
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>, // System program for transfers
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        // Ensure the game is initialized and in the InProgress state
        require!(
            game_account.game_state == GameState::InProgress,
            StakeDeckError::GameNotInProgress
        );

        // Check if the player is part of the game
        require!(
            game_account
                .players
                .iter()
                .any(|player| player.pubkey == ctx.accounts.user.key()),
            StakeDeckError::PlayerNotInGame // Change the error to indicate the player is not in the game
        );
        // Deduct SOL from the player and transfer to the vault
        let player_lamports = **ctx.accounts.user.lamports.borrow();
        require!(
            player_lamports >= amount,
            StakeDeckError::InsufficientFundsForBetting
        );

        // Transfer SOL to the vault
        let transfer_instruction = system_instruction::transfer(
            ctx.accounts.user.key,
            &ctx.accounts.vault.key(),
            amount,
        );

        invoke(
            &transfer_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Update the game account with the new bet amount
        if let Some(player) = game_account.players.iter_mut().find(|player| player.pubkey == ctx.accounts.user.key()) {
            // Update the player's bet amount
            player.bet_amount += amount; // Increment the player's bet
        } else {
            return Err(StakeDeckError::PlayerNotInGame.into());
        }

        Ok(())
    }
}