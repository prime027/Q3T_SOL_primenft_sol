use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use solana_program::program::invoke;
use solana_program::system_instruction;

use crate::error::StakeDeckError;

#[derive(Accounts)]
pub struct StartGame<'info> {
    #[account(
    mut,
    has_one = player1,
    has_one = player2,
)]
pub players_account: Account<'info, PlayersAccount>,

#[account(mut)]
pub vault: SystemAccount<'info>,

#[account(mut)]
pub player1: Signer<'info>,

#[account(mut)]
pub player2: Signer<'info>,
pub system_program: Program<'info, System>,

}

#[account]
pub struct PlayersAccount {
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub bet_amount: u64,
    pub game_state: GameState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GameState {
    WaitingForPlayers,
    InProgress,
    Completed,
}

impl<'info> StartGame<'info> {
    pub fn start_game(&mut self) -> Result<()> {
        let players_account = &mut self.players_account;
        // Ensure the game is not already in progress
        require!(
            matches!(players_account.game_state, GameState::WaitingForPlayers),
            StakeDeckError::GameAlreadyInProgress
        );

        let vault = &mut self.vault;
        let player1 = &self.player1;
        let player2 = &self.player2;
        let bet_amount = players_account.bet_amount;


        // Check if player1 has enough SOL
        require!(
            player1.lamports() >= bet_amount,
            StakeDeckError::InsufficientFundsForBetting
        );

        // Check if player2 has enough SOL
        require!(
            player2.lamports() >= bet_amount,
            StakeDeckError::InsufficientFundsForBetting
        );

        // Transfer SOL from player1 to the vault
        invoke(
            &system_instruction::transfer(
                &player1.key(),
                &vault.key(),
                bet_amount
            ),
            &[
                player1.to_account_info(),
                vault.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        // Transfer SOL from player2 to the vault
        invoke(
            &system_instruction::transfer(
                &player2.key(),
                &vault.key(),
                bet_amount
            ),
            &[
                player2.to_account_info(),
                vault.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        // Set the game state to InProgress
        players_account.game_state = GameState::InProgress;
        Ok(())
    }
}