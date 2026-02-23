use anchor_lang::prelude::*;

#[derive(InitSpace)]  // Anchor calculates the size for you
#[account]
pub struct Treasury {
    pub bump: u8,
}
