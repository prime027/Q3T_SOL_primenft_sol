use anchor_lang::prelude::*;
pub mod contexts;
pub use contexts::*;
pub mod state;

declare_id!("FRVB1SvnvCrFy8L7m8K6FnpjA2LC286rHrEDMvpCHMxo");

#[program]
pub mod amm {
    use super::*;


    // initialize a pool
    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, amount_x: u64, amount_y: u64) -> Result<()> {
        // save_config
        ctx.accounts.save_config(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp)?;
        ctx.accounts.deposit(amount_x, true)?;
        ctx.accounts.deposit(amount_y, false)?;
        ctx.accounts.mint_lp_tokens(amount_x, amount_y)
    }

    // // Deposit liquidity to mint LP tokens 
    // pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
    //     // deposit_tokens (amount)
    //     // mint_lp_tokens (amount)
    // }

    // // Burn LP tokens to withdraw liquidity
    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
    //     // deposit_tokens (amount)
    //     // withdraw_tokens (amount)
    // }

    // pub fn swap(ctx: Context<Swap>, amount: u64, min_receive: u64, is_x: bool) -> Result<()> {
    //     // deposit_tokens ()
    //     // mint_lp_tokens ()
    // }
}


