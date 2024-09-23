use anchor_lang::prelude::*;

use crate::GameConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"config"],
        bump,
        space = GameConfig::INIT_SPACE,
    )]
    pub config: Account<'info, GameConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(&mut self, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner(GameConfig {
            min_bet: 5_000_000_000, // You should set an appropriate minimum bet value
            max_players: 2, // Assuming 2 players based on the previous code
            fee_percentage: 5, // Set an appropriate fee percentage
            payout_percentage: 95, // Set an appropriate payout percentage
            bump: bumps.config,
        });
        Ok(())
    }
}
