use anchor_lang::prelude::*;

#[error_code]
pub enum StakeDeckError {
    #[msg("Invalid player account")]
    InvalidPlayerAccount,
    #[msg("Invalid configuration")]
    InvalidConfig,
    #[msg("Bet amount must be greater than zero.")]
    InvalidBetAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("The game is already in progress.")]
    GameAlreadyInProgress,
    #[msg("Insufficient funds for betting.")]
    InsufficientFundsForBetting,
    #[msg("Game has not started")]
    GameNotWaitingForPlayers,
    #[msg("Max players reached")]
    MaxPlayersReached,
    #[msg("Player already joined")]
    PlayerAlreadyJoined,
    #[msg("Game is not in progress.")]
    GameNotInProgress,
    #[msg("Player is not in the game.")]
    PlayerNotInGame,
    // Add more custom errors as needed
}
