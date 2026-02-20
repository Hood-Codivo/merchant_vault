use anchor_lang::prelude::*;

#[account]
pub struct Merchant {
    pub authority: Pubkey,
    pub vault: u8,
}

impl Merchant {
    pub const LEN: usize = 32 + 1;
}