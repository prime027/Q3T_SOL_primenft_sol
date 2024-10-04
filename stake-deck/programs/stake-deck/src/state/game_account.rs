use anchor_lang::prelude::*;


#[account]
pub struct GameAccount {
    pub players: Vec<Player>, // Dynamic list of players
    pub min_bet: u64,          // Minimum bet configuration
    pub max_players: u8,       // Maximum number of players
    pub fee_percentage: u8,     // Fee percentage for the game
    pub payout_percentage: u8,  // Payout percentage for the game
    pub bet_amount: u64,       // Total bet amount
    pub game_state: GameState,  // Current state of the game
    // pub admin: Pubkey,          //Admin of the game 
    pub bump: u8,               // Bump seed for account
}

impl GameAccount {
    pub const INIT_SPACE: usize = 8 + // discriminator
        4 + // players vector length (u32)
        32 * 4 + // players (Player) - assuming a max of 4 for space allocation
        8 + // min_bet (u64)
        1 + // max_players
        1 + // fee_percentage
        1 + // payout_percentage
        8 + // bet_amount (u64)
        //32 + // admin (Pubkey)
        1;  // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    WaitingForPlayers,
    InProgress,
    Completed,
    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Player {
    pub pubkey: Pubkey, // The public key of the player
    pub bet_amount: u64, // The amount the player has bet
}


// #[derive(borshSerialize, AnchorDeserialize, Clone)]
// pub struct InitializeGameBumps {
//     pub game_account: u8,
//     pub vault_state: u8,
//     pub vault: u8,
// }

#[account]
pub struct VaultState {
    pub owner: Pubkey, // Owner of the vault (game account)
    pub balance: u64,   // Current balance in the vault
    pub vault_bump: u8,
    pub state_bump: u8,
}

// impl GameAccount {
//     pub const INIT_SPACE: usize = 8 + // discriminator
//                               32 + // player_1
//                               32 + // player_2
//                               8 + // player_1_bet
//                               8 + // player_2_bet
//                               1 + // game_state (enum)
//                               1; // bump
// }