use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::{
    merchant::Merchant,
    vault::Vault,
    config::Config,
    payment::Payment,
    treasury::Treasury,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct PayWithSol<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"merchant", merchant.authority.as_ref()],
        bump = merchant.bump,
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        mut,
        seeds = [b"vault", merchant.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = payer,
        space = Payment::DISCRIMINATOR.len() + Payment::INIT_SPACE,
        seeds = [b"payment", merchant.key().as_ref(), &merchant.payment_count.to_le_bytes()],
        bump,
    )]
    pub payment: Account<'info, Payment>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayWithSol>, amount: u64) -> Result<()> {
    require!(amount > 0, crate::errors::VaultError::InvalidPaymentAmount);

    let config = &ctx.accounts.config;
    let payer = &ctx.accounts.payer;

    let merchant = &mut ctx.accounts.merchant;

    // If multiplication overflows, return MathOverflow instead of crashing
    let fee = amount
        .checked_mul(config.platform_fee_bps as u64)
        .ok_or(crate::errors::VaultError::MathOverflow)?
        / 10_000;

    // If subtraction underflows, return MathUnderflow instead of crashing
    let merchant_amount = amount
        .checked_sub(fee)
        .ok_or(crate::errors::VaultError::MathUnderflow)?;

    // Transfer to Merchant Vault
    let cpi_ctx_vault = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: payer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx_vault, merchant_amount)?;

    // Transfer fee to Treasury
    let cpi_ctx_treasury = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: payer.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx_treasury, fee)?;

    // Record payment
    let payment = &mut ctx.accounts.payment;
    payment.payer = payer.key();
    payment.mint = Pubkey::default(); // SOL
    payment.amount = amount;
    payment.fee_amount = fee;
    payment.timestamp = Clock::get()?.unix_timestamp;
    payment.bump = ctx.bumps.payment;

    merchant.payment_count = merchant.payment_count.checked_add(1).ok_or(crate::errors::VaultError::MathOverflow)?;

    Ok(())
}