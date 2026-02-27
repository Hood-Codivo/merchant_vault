use anchor_lang::prelude::*;

#[derive(InitSpace)]  // Anchor calculates the size for you
#[account]
pub struct Payment {
    pub payer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub fee_amount: u64,
    pub timestamp: i64,
    pub bump: u8,
}
