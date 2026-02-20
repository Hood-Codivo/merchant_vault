use anchor_lang::prelude::*;

#[account]
pub struct Payment {
    pub payer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub fee_bps: u16,
    pub bump: u8,
}

impl Payment {
    pub const LEN: usize = 32 + 32 + 8 + 2 + 1; // size of the Payment account (Pubkey + Pubkey + u64 + u16 + u8)
}