use anchor_lang::prelude::*;
use crate::state::{Listing, ListingArgs};

use anchor_spl::token_interface::Mint;
use crate::constants::SEED_LISTING_ACCOUNT;

#[derive(Accounts)]
#[instruction(args: ListingArgs)]
pub struct UpdateListing<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    price_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
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

    pub fn update_listing(&mut self, args: ListingArgs) -> Result<()> {
        self.listing.price = args.price.expect("price is missing for this listing");

        Ok(())
    }
}
