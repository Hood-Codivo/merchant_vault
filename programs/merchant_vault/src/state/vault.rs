
use anchor_lang::prelude::*;

#[derive(InitSpace)]  // Anchor calculates the size for you
#[account]
pub struct Vault {
    pub merchant: Pubkey,
    pub bump: u8,
}

