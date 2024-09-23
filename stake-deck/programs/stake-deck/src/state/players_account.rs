use anchor_lang::prelude::*;

#[account]
pub struct PlayersAccount {
    pub player_1: Pubkey,
    pub player_2: Pubkey,
    pub bet_amount: u64,
    pub game_state: u8,
    pub bump: u8,
}

impl Space for PlayersAccount {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1 + 1;
}