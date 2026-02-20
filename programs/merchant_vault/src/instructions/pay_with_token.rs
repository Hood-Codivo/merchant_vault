use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{
    merchant::Merchant,
    vault::Vault,
    config::Config,
    payment::Payment,
    treasury::Treasury,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct PayWithToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [b"merchant", merchant.authority.as_ref()],
        bump = merchant.bump,
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        seeds = [b"vault", merchant.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    // Payer token account
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,

    // Merchant vault ATA (owned by Vault PDA)
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    // Treasury ATA (owned by Treasury PDA)
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        space = 8 + Payment::LEN,
        seeds = [
            b"payment",
            merchant.key().as_ref(),
            payer.key().as_ref(),
        ],
        bump
    )]
    pub payment: Account<'info, Payment>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayWithToken>, amount: u64) -> Result<()> {
    require!(amount > 0, crate::errors::VaultError::InvalidPaymentAmount);

    let config = &ctx.accounts.config;

    let fee = amount
        .checked_mul(config.platform_fee_bps as u64)
        .unwrap()
        / 10_000;

    let merchant_amount = amount.checked_sub(fee).unwrap();

    // Transfer to Merchant Vault
    let cpi_ctx_vault = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token::transfer(cpi_ctx_vault, merchant_amount)?;

    // Transfer fee to Treasury
    let cpi_ctx_treasury = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token::transfer(cpi_ctx_treasury, fee)?;

    // Record payment
    let payment = &mut ctx.accounts.payment;
    payment.payer = ctx.accounts.payer.key();
    payment.mint = ctx.accounts.payer_token_account.mint;
    payment.amount = amount;
    payment.fee_amount = fee;
    payment.timestamp = Clock::get()?.unix_timestamp;
    payment.bump = ctx.bumps.payment;

    Ok(())
}
