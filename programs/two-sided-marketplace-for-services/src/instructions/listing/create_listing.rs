use anchor_lang::prelude::*;
use crate::state::{Listing, ListingArgs};
use anchor_spl::token_interface::Mint;
use mpl_core::{
    instructions::{AddPluginV1Cpi, AddPluginV1InstructionArgs},
    types::{PluginAuthority, TransferDelegate, Plugin},
};
use crate::constants::SEED_LISTING_ACCOUNT;


#[derive(Accounts)]
#[instruction(args: ListingArgs)]
pub struct CreateListing<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    price_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        space = Listing::INIT_SPACE,
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


impl<'info> CreateListing<'info> {

    pub fn create_listing(&mut self, args: ListingArgs, bumps: &CreateListingBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            lister: self.payer.key(),
            mint: self.price_mint.key(),
            price: args.price.expect("price is missing for this listing"),
            seed: args.seed,
            bump: bumps.listing,
        });

        Ok(())
    }

    pub fn delegate_transfer_authority(&mut self) -> Result<()> {

        let transfer_delegate_plugin = AddPluginV1InstructionArgs {
            plugin: Plugin::TransferDelegate(TransferDelegate{}),
            init_authority: Some(PluginAuthority::Address { address: self.listing.key() }),
        };

        AddPluginV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            authority: self.authority.as_deref(),
            payer: &self.payer.to_account_info(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: transfer_delegate_plugin,
        }
        .invoke()?;

        Ok(())

    }

}



