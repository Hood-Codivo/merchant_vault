pub mod state;
pub mod instructions;
pub mod errors;

use anchor_lang::prelude::*;
// Bring the structs into the top-level scope so #[program] can find them easily
use crate::instructions::*;

declare_id!("2vm8xz2TFAQH2N6ijqLNRt6J7bh8h7hy92hYM19Vy4SD");

#[program]
pub mod merchant_vault {
    use super::*;

    // Note: We call instructions::handler because the mod and the function 
    // often share names.
    pub fn initialize_config(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
        instructions::initialize_config::handler(ctx, fee_bps)
    }

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        instructions::initialize_treasury::handler(ctx)
    }

    pub fn initialize_merchant(ctx: Context<RegisterMerchant>) -> Result<()> {
        instructions::register_merchant::handler(ctx)
    }

    pub fn pay_with_sol(ctx: Context<PayWithSol>, amount: u64) -> Result<()> {
        instructions::pay_with_sol::handler(ctx, amount)
    }

    pub fn pay_with_token(ctx: Context<PayWithToken>, amount: u64) -> Result<()> {
        instructions::pay_with_token::handler(ctx, amount)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        instructions::withdraw_sol::handler(ctx, amount)
    }
}