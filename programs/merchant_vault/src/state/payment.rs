use anchor_lang::prelude::*;

#[account]
pub struct Payment {
    pub payer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub fee_amount: u64,
    pub timestamp: i64,
    pub bump: u8,
}

impl Payment {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 1;
}
