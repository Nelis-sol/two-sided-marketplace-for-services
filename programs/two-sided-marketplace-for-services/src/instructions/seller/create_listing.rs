use anchor_lang::prelude::*;
use crate::state::ListingState;

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub listing: Account<'info, ListingState>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateListing<'info> {

    // bumps: &CreateListingBumps
    pub fn create_listing(&mut self) -> Result<()> {
        Ok(())
    }

}