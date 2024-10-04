use anchor_lang::prelude::*;
use crate::{error::StakeDeckError, state::{GameState, PlayersAccount}, GameAccount, Player,  VaultState};



#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub first_player: Signer<'info>,
    #[account(
        init,
        payer = first_player,
        seeds = [b"player", first_player.key().as_ref()],
        bump,
        space = GameAccount::INIT_SPACE,
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(
        init,
        payer = first_player,
        seeds = [b"vault", game_account.key().as_ref()],
        bump,
        space = 8 + 32 + 8 + 1 + 1, // Adjust space for owner and balance
    )]
    pub vault: Account<'info, VaultState>,
    // #[account(
    //     seeds = [b"vault", vault_state.key().as_ref()],
    //     bump,
    // )]
    // pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'info> {
    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        //bumps: &InitializeGameBumps,
        min_bet: u64,
        max_players: u8, // Dynamic max players
        fee_percentage: u8,
        payout_percentage: u8,
        admin: Pubkey
    ) -> Result<()> {
            let first_player: &Signer<'_> = &ctx.accounts.first_player;  
            let game_account = &mut ctx.accounts.game_account; 
            // let bump = ctx.bumps.game_account;
            let vault = &mut ctx.accounts.vault;

        if first_player.key() == Pubkey::default() {
            return Err(StakeDeckError::InvalidPlayerAccount.into());
        }

        let player = Player {
            pubkey: first_player.key(),
            bet_amount: 0, // Initial bet amount is zero
        };

        game_account.players.push(player);
        game_account.min_bet = min_bet;
        game_account.max_players = max_players;
        game_account.fee_percentage = fee_percentage;
        game_account.payout_percentage = payout_percentage;
        game_account.bet_amount = 0;
        game_account.game_state = GameState::WaitingForPlayers; // Initial state
        // game_account.bump =bump;
        // game_account.admin = admin;


        // Vault
        vault.owner = first_player.key();
        // vault_state.vault_bump = bumps.vault;
        // vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}