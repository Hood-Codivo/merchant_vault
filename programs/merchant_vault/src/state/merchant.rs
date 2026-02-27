use anchor_lang::prelude::*;

#[derive(InitSpace)]  // Anchor calculates the size for you
#[account]
pub struct Merchant {
    pub authority: Pubkey,
    pub bump: u8,
    pub payment_count: u64,
}

impl Merchant {
    pub const INIT_SPACE: usize = 32 + 1 + 8;
}

