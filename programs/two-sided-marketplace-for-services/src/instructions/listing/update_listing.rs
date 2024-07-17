use anchor_lang::prelude::*;
use crate::{
    errors::MarketplaceError, state::{Listing, ListingArgs},
    constants::SEED_LISTING_ACCOUNT,
};

use anchor_spl::token_interface::Mint;


#[derive(Accounts)]
#[instruction(args: ListingArgs)]
pub struct UpdateListing<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        mut,
        // Listing PDA requires signer/payer as seed and a identifier/seed
        seeds = [SEED_LISTING_ACCOUNT, payer.key().as_ref(), args.seed.to_le_bytes().as_ref()],
        bump
    )]
    listing: Account<'info, Listing>,
    system_program: Program<'info, System>,
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,
    /// The owner or delegate of the asset.
    pub authority: Option<Signer<'info>>,
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

impl<'info> UpdateListing<'info> {

    // Update the listing where the service (NFT) is offered
    pub fn update_listing(&mut self, args: ListingArgs) -> Result<()> {
        
        // Currently, only the price is allowed to be updated
        self.listing.price = args.price.ok_or(MarketplaceError::MissingPrice)?;

        Ok(())
    }
}
