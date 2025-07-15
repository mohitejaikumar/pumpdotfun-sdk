use anchor_lang::prelude::*;


#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Slippage")]
    InvalidSlippage,
    #[msg("Failed to fetch global state from chain")]
    GlobalNotFound,
    #[msg("Failed to deserialize global state")]
    DeserializationError,
    #[msg("Overflow")]
    Overflow,
}
