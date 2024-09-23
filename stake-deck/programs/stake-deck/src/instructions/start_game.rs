use anchor_lang::prelude::*;
//use anchor_lang::system_program::{transfer, Transfer};
use solana_program::program::invoke;
use solana_program::system_instruction;

use crate::error::StakeDeckError;
use crate::{GameAccount, GameState, Player, Vault};

#[derive(Accounts)]
pub struct StartGame<'info> {
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>, // The game account being joined
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", game_account.key().as_ref()], // Ensure vault is seeded with game_account
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

impl<'info> StartGame<'info> {
    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let bet_amount = game_account.bet_amount;

        // Ensure the game is initialized and in the WaitingForPlayers state
        require!(
            game_account.game_state == GameState::WaitingForPlayers,
            StakeDeckError::GameNotWaitingForPlayers
        );

        // Check if the player can join
        require!(
            game_account.players.len() < game_account.max_players as usize,
            StakeDeckError::MaxPlayersReached
        );

        // Check if the player is already in the game
        require!(
            !game_account
                .players
                .iter()
                .any(|player| player.pubkey == ctx.accounts.player.key()),
            StakeDeckError::PlayerAlreadyJoined
        );

        // Deduct SOL from the player and transfer to the vault
        let player_lamports = **ctx.accounts.player.lamports.borrow();
        require!(
            player_lamports >= bet_amount,
            StakeDeckError::InsufficientFundsForBetting
        );

        // Add the new player
        let new_player = Player {
            pubkey: ctx.accounts.player.key(),
            bet_amount: 0, // Initial bet amount is zero
        };
        game_account.players.push(new_player);

        // // Transfer SOL to the vault
        // let transfer_instruction = system_instruction::transfer(
        //     ctx.accounts.player.key,
        //     &ctx.accounts.vault.key(),
        //     bet_amount,
        // );

        // invoke(
        //     &transfer_instruction,
        //     &[
        //         ctx.accounts.player.to_account_info(),
        //         ctx.accounts.vault.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;

        // Update game state if the maximum number of players is reached
        if game_account.players.len() == game_account.max_players as usize {
            game_account.game_state = GameState::InProgress; // Change state to in progress
        }

        Ok(())
    }
}
