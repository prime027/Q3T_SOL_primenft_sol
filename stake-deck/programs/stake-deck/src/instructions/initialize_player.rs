use anchor_lang::prelude::*;
use crate::{state::PlayersAccount, error::StakeDeckError};

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        init,
        payer = player,
        seeds = [b"player", player.key().as_ref()],
        bump,
        space = PlayersAccount::INIT_SPACE,
    )]
    pub player_account: Account<'info, PlayersAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePlayer<'info> {
    pub fn initialize_player(&mut self, bump: u8) -> Result<()> {
        if self.player.key() == Pubkey::default() {
            return Err(StakeDeckError::InvalidPlayerAccount.into());
        }

        self.player_account.set_inner(PlayersAccount {
            player_1: self.player.key(),
            player_2: Pubkey::default(),
            bet_amount: 0,
            game_state: 0,
            bump,
        });

        Ok(())
    }
}