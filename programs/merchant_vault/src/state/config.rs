use anchor_lang::prelude::*;

#[derive(InitSpace)]  // Anchor calculates the size for you
#[account]
pub struct Config {
    pub platform_fee_bps: u16,
    pub bump: u8,
}
