use anchor_lang::prelude::*;

#[account]
pub struct GameAccount {
    pub player_1: Pubkey,
    pub player_2: Pubkey,
    pub player_1_bet: u64,
    pub player_2_bet: u64,
    pub game_state: GameState,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    WaitingForPlayers,
    InProgress,
    Completed,
    
}

impl GameAccount {
    pub const INIT_SPACE: usize = 8 + // discriminator
                              32 + // player_1
                              32 + // player_2
                              8 + // player_1_bet
                              8 + // player_2_bet
                              1 + // game_state (enum)
                              1; // bump
}