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
    pub fn initialize_player(ctx: Context<InitializePlayer>, bump: u8) -> Result<()> {
        ctx.accounts.initialize_player(bump)?;

        Ok(())
    }

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        ctx.accounts.initialize_config(&ctx.bumps)?;
        Ok(())
    }
    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        ctx.accounts.place_bet(amount)?;
        Ok(())
    }
    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        ctx.accounts.start_game()?;
        Ok(())
    }
}
