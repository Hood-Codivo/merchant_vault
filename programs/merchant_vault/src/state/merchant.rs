use anchor_lang::prelude::*;

#[derive(InitSpace)]  // Anchor calculates the size for you
#[account]
pub struct Merchant {
    pub authority: Pubkey,
    pub bump: u8,
}

