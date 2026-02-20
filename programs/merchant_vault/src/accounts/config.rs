use anchor_lang::prelude::*;

 #[account]
pub struct Config {
    pub platform_fee_bps: u16, // fee in basic points (bps)
    pub bump: u8, // bump for the config PDA
}

impl Config {
    pub const LEN: usize = 2 + 1; // size of the Config account (u16 + u8)
}