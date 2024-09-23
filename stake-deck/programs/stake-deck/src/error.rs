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
    // Add more custom errors as needed
}
