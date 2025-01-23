use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access.")]
    Unauthorized,
    #[msg("Invalid duration.")]
    InvalidDuration,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
}