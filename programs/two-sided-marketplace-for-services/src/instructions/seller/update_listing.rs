use anchor_lang::prelude::*;
use crate::state::ListingState;

#[derive(Accounts)]
pub struct UpdateListing<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub listing: Account<'info, ListingState>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateListing<'info> {

    pub fn update_listing(&mut self) -> Result<()> {
        Ok(())
    }

}