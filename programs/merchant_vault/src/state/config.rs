use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub platform_fee_bps: u16,
    pub bump: u8,
}

impl Config {
    pub const LEN: usize = 2 + 1;
}
