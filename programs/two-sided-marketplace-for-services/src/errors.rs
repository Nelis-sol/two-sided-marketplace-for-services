use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Price is missing for listing")]
    MissingPrice,
    #[msg("Underflow")]
    Underflow,
    #[msg("Overflow")]
    Overflow,
}
