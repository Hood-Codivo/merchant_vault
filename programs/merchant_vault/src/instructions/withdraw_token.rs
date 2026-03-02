use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{merchant::Merchant, vault::Vault};
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"merchant", authority.key().as_ref()],
        bump = merchant.bump,
        has_one = authority @ VaultError::Unauthorized,
    )]
    pub merchant: Account<'info, Merchant>,

    // ✅ mut required — vault is the signer authority for the token transfer
    #[account(
        mut,
        seeds = [b"vault", merchant.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawToken>, amount: u64) -> Result<()> {
    // ✅ require! not requied!
    require!(amount > 0, VaultError::InvalidPaymentAmount);

    require!(
        ctx.accounts.vault_token_account.amount >= amount,
        VaultError::InsufficientFunds
    );

    let merchant_key = ctx.accounts.merchant.key();

    // ✅ comma not semicolon at end of seeds array
    let seeds = &[
        b"vault" as &[u8],
        merchant_key.as_ref(),
        &[ctx.accounts.vault.bump],
    ];

    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            // ✅ authority_token_account not autority_token_account
            to: ctx.accounts.authority_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}