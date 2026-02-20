use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized: Only the merchant can perform this action.")]
    Unauthorized,
    #[msg("Insufficient funds in the vault.")]
    InsufficientFunds,
    #[msg("Payment amount must be greater than zero")]
    InvalidPaymentAmount,
}