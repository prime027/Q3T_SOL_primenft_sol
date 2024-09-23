pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3jpEDmkjLcfFqBZBj6VMq9BnNpXgwJY7v148wdWFCN1F");

#[program]
pub mod stake_deck {
    use super::*;
    pub fn initialize_game(ctx: Context<InitializeGame>, min_bet: u64, max_players: u8, fee_percentage: u8, payout_percentage: u8) -> Result<()> {
        InitializeGame::initialize_game(ctx, min_bet, max_players, fee_percentage, payout_percentage)
    }

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        InitializeConfig::initialize_config(ctx)
    }

    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        StartGame::start_game(ctx)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        PlaceBet::place_bet(ctx, amount)
    }
}
