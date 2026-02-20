pub mod initialize_config;
pub mod register_merchant;
pub mod pay_with_sol;
pub mod pay_with_token;
pub mod initialize_treasury;
pub mod withdraw_sol;

// Use * to bring everything (Accounts and handlers) into the instructions namespace
pub use initialize_config::*;
pub use register_merchant::*;
pub use pay_with_sol::*;
pub use pay_with_token::*;
pub use initialize_treasury::*;
pub use withdraw_sol::*;