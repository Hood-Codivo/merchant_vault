use anchor_lang::prelude::*;
use crate::state::{merchant::Merchant, vault::Vault, config::Config};

#[derive(Accounts)]
pub struct RegisterMerchant<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        space = 8 + Merchant::LEN,
        seeds = [b"merchant", authority.key().as_ref()],
        bump,
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        init,
        payer = authority,
        space = 8 + Vault::LEN,
        seeds = [b"vault", merchant.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterMerchant>) -> Result<()> {
    let merchant = &mut ctx.accounts.merchant;
    merchant.authority = ctx.accounts.authority.key();
    merchant.bump = ctx.bumps.merchant;

    let vault = &mut ctx.accounts.vault;
    vault.merchant = merchant.key();
    vault.bump = ctx.bumps.vault;

    Ok(())
}