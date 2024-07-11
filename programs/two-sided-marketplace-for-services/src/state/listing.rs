use anchor_lang::prelude::*;

#[account]
pub struct ListingState {
    pub id: u64,
    pub bump: u8,
}