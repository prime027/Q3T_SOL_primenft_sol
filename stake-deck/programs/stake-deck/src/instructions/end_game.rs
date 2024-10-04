
use anchor_lang::system_program::{transfer, Transfer};


use anchor_lang::prelude::*;
use crate::{error::StakeDeckError, state::{GameState, GameAccount, VaultState}};

#[derive(Accounts)]
pub struct EndGame<'info> {
    // #[account(mut)]
    // pub admin: Signer<'info>, // The admin ending the game
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>, // The game account
    // #[account(
    //     mut,
    //     seeds = [b"vault", game_account.key().as_ref()], // Ensure vault is seeded with game_account
    //     bump,
    //     close = first_player
    // )]
    // pub vault: Account<'info, Vault>, // The vault account holding the funds
   
    #[account(
        mut,
        seeds = [b"vault", game_account.key().as_ref()],  // Ensure vault is seeded with game_account
        bump,
    )]
    pub vault: Account<'info, VaultState>,
    // #[account(
    //     seeds = [b"vault", vault_state.key().as_ref()],
    //     bump = vault_state.vault_bump,
    // )]
    // pub vault: SystemAccount<'info>,
   
    /// Lets test this for now
    //pub winner: UncheckedAccount<'info>,// The winner's account
    /// CHECK: Admin public key is fixed and trusted
    #[account(
        mut
        // address = game_account.admin
    )] // Admin (game creator) fixed public key
    pub admin: Signer<'info>, // Admin's account (fixed creator)
    /// CHECK:
    pub first_player: UncheckedAccount<'info>, // The first player who initialized the vault
    /// CHECK:
    #[account(
        mut
    )]
    pub winner: Signer<'info>,
    pub system_program: Program<'info, System>, // Required for system instructions


    //pub vault_bump: u8,
}

impl<'info> EndGame<'info> {
    pub fn end_game(ctx: Context<EndGame>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let vault = &mut ctx.accounts.vault;
        let winner =  &ctx.accounts.winner;
        let admin = &mut ctx.accounts.admin;
        // let first_player = &mut ctx.accounts.first_player;

        require!(
            game_account.game_state == GameState::InProgress,
            StakeDeckError::GameNotInProgress
        );

        //  // Calculate the amounts to distribute
        // let vault_balance = **vault.to_account_info().lamports.borrow();
        // let fee_amount = vault_balance * game_account.fee_percentage as u64 / 100; // Fee based on fee_percentage
        // let winner_amount = vault_balance - fee_amount; // Remaining balance goes to the winner

        // **vault.to_account_info().lamports.borrow_mut() -= vault_balance;
        // **winner.to_account_info().lamports.borrow_mut() += vault_balance;

        let game_account_key_bytes = game_account.key().to_bytes();

        let signer_seeds: &[&[u8]] = &[
                b"vault",
                &game_account_key_bytes,
                &[vault.vault_bump],
            
        ];

        let signer_seeds_full: &[&[&[u8]]] = &[signer_seeds];

        // Transfer 5% of the vault's balance to the admin
        let vault_balance = **vault.to_account_info().lamports.borrow();
        let fee_amount = vault_balance * game_account.fee_percentage as u64 / 100;
        let winner_amount = vault_balance - fee_amount;

        // Create transfer context for admin
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let transfer_admin = Transfer {
            from: vault.to_account_info(),
            to: admin.to_account_info(),
        };
        let cpi_ctx_admin = CpiContext::new_with_signer(
            cpi_program.clone(),
            transfer_admin,
            &signer_seeds_full,
        );
        transfer(cpi_ctx_admin, fee_amount)?;

        //Create transfer context for winner
        let transfer_winner = Transfer {
            from: vault.to_account_info(),
            to: winner.to_account_info(),
        };
        let cpi_ctx_winner = CpiContext::new_with_signer(
            cpi_program.clone(),
            transfer_winner,
            &signer_seeds_full,
        );
        transfer(cpi_ctx_winner, winner_amount)?;

        
         //let bump = ctx.bumps.vault;
        //  let seeds = &[&[b"vault", ctx.accounts.vault_state.to_account_info().key.as_ref()]];
        //  let signer_seeds = &[&seeds[..]];

    //     let vault_signer_seeds: &[&[u8]] = &[
    //         b"vault",
    //         ctx.accounts.vault_state.to_account_info().key.as_ref(),
    //         &[ctx.accounts.vault_state.vault_bump]
    //     ];

    //     // Now bind the seeds to an array to extend their lifetime
    //     let seeds = &[vault_signer_seeds]; // Store seeds to ensure lifetime

    //      let cpi_program = ctx.accounts.system_program.to_account_info().clone();

    //     // Transfer 95% of the vault's balance to the winner
    //      let cpi_accounts = Transfer {
    //          from: vault.to_account_info(),
    //          to: winner.to_account_info(),
    //      };
      
 
    //      let cpi_ctx = CpiContext::new_with_signer(
    //         cpi_program.clone(), 
    //         cpi_accounts, 
    //         //signer_seeds
    //         // &[vault_signer_seeds]
    //         seeds,
    //     );
 
    //     transfer(cpi_ctx, winner_amount)?;

    //     // Transfer 5% of the vault's balance to the admin

    //     let cpi_accounts = Transfer {
    //         from: vault.to_account_info(),
    //         to: admin.to_account_info(),
    //     };
     

    //     let cpi_ctx = CpiContext::new_with_signer(
    //         cpi_program, 
    //         cpi_accounts, 
    //         //signer_seeds
    //         // &[vault_signer_seeds],
    //         seeds,
    //     );

    //    transfer(cpi_ctx, fee_amount)?;

        // // Transfer 95% of the vault's balance to the winner

        // // Transfer 95% of the vault's balance to the winner
        // let winner_transfer_instruction = system_instruction::transfer(
        //     ctx.accounts.vault.to_account_info().key,
        //     ctx.accounts.winner.to_account_info().key,
        //     winner_amount, // Amount to transfer
        // );

        // // Invoke the transfer instruction for the winner
        // invoke_signed(
        //     &winner_transfer_instruction,
        //     &[
        //         ctx.accounts.vault.to_account_info(),
        //         ctx.accounts.winner.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     vault_signer,
        // )?;
        // // **vault.to_account_info().lamports.borrow_mut() -= winner_amount;
        // // **winner.to_account_info().lamports.borrow_mut() += winner_amount;
        // // Transfer 5% of the vault's balance to the admin
  
        // let admin_transfer_instruction = system_instruction::transfer(
        //     ctx.accounts.vault.to_account_info().key,
        //     ctx.accounts.admin.to_account_info().key,
        //     fee_amount, // Fee amount to transfer
        // );

        // // Invoke the transfer instruction for the admin
        // invoke_signed(
        //     &admin_transfer_instruction,
        //     &[
        //         ctx.accounts.vault.to_account_info(),
        //         ctx.accounts.admin.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     vault_signer,
        // )?;
        // Transfer 5% of the vault's balance to the admin (fixed pubkey)
        // **vault.to_account_info().lamports.borrow_mut() -= fee_amount;
        // **admin.to_account_info().lamports.borrow_mut() += fee_amount;

        // Set the game state to Finished
        game_account.game_state = GameState::Completed;

        Ok(())
    }
}