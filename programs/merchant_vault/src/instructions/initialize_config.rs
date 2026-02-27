use anchor_lang::prelude::*;
use crate::state::config::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
    let config = &mut ctx.accounts.config;

    require!(fee_bps <= 1000, ErrorCode::InvalidFee);

    config.platform_fee_bps = fee_bps;
    config.bump = ctx.bumps.config;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid fee percentage")]
    InvalidFee,
}