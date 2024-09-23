use anchor_lang::prelude::*;

#[account]
pub struct GameConfig {
    pub min_bet: u64,  // Changed to u64 for $5 in lamports
    pub max_players: u8,
    pub fee_percentage: u8,
    pub payout_percentage: u8,
    pub bump: u8,
}

impl GameConfig {
    pub const MIN_BET: u64 = 5_000_000_000;  // $5 in lamports (1 SOL = 1_000_000_000 lamports)
    pub const MAX_PLAYERS: u8 = 2;
    pub const FEE_PERCENTAGE: u8 = 5;
    pub const PAYOUT_PERCENTAGE: u8 = 95;  // Winner takes all

    pub const INIT_SPACE: usize = 8 + // discriminator
        8 + // min_bet (u64)
        1 + // max_players
        1 + // fee_percentage
        1 + // payout_percentage
        1;  // bump
}