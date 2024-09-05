use anchor_lang::prelude::*;

#[error_code]
pub enum DiceError {
    #[msg("Bump error")]
    BumpError,
    #[msg("Overflow")]
    Overflow,
    #[msg("Minimum bet is 0.01 Sol")]
    MinimumBet,
    #[msg("Maximum bet exceeded")]
    MaximumBet,
    #[msg("Minimum roll is 2")]
    MinimumRoll,
    #[msg("Maximun roll is 96")]
    MaximumRoll,
    #[msg("Timeout not yet reached")]
    TimeoutNotreached,
    #[msg("Invalid Ed25519 program")]
    Ed25519Program,
    #[msg("Ed25519 message error")]
    Ed25519Message,
    #[msg("Invalid number of Ed25519 accounts")]
    Ed25519Accounts,
    #[msg("Invalid Ed25519 data length")]
    Ed25519DataLength,
    #[msg("Invalid Ed25519 header")]
    Ed25519Header,
    #[msg("Invalid Ed25519 public key")]
    Ed25519Pubkey,
    #[msg("Invalid Ed25519 signature")]
    Ed25519Signature,
}
