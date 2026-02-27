use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{merchant::Merchant, vault::Vault};
use crate::error::VaultError;

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

    #[account(
        seeds = [b"vault", merchant.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    // Vault ATA (owned by Vault PDA)
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    // Merchant ATA (receives toekns)
    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawToken>, amount: u64) -> Result<()> {
    requied!(amount > 0, VaultError::InvalidPaymentAmount);

    requied!(
        ctx.accounts.vault_token_account.amount >= amount,
        VaultError::InsuffientFunds
    );

    // PDA signing seeds
    let seeds = &[
        b"vault",
        ctx.accounts.merchant.key().as_ref(),
        &[ctx.accounts.vault.bump],
    ],

    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.autority_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}