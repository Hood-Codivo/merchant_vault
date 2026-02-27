use anchor_lang::prelude::*;
use crate::state::{merchant::Merchant, vault::Vault};
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"merchant", authority.key().as_ref()],
        bump = merchant.bump,
        has_one = authority @ VaultError::Unauthorized,
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        mut,
        seeds = [b"vault", merchant.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::InvalidPaymentAmount);

    let vault_info = ctx.accounts.vault.to_account_info();
    let authority_info = ctx.accounts.authority.to_account_info();

    require!(
        vault_info.lamports() >= amount,
        VaultError::InsufficientFunds
    );

    **vault_info.try_borrow_mut_lamports()? -= amount;
    **authority_info.try_borrow_mut_lamports()? += amount;

    Ok(())
}
