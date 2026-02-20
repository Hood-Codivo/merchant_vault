use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub merchant: Pubkey,
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 32 + 1;
}
