use anchor_lang::prelude::*;
use crate::{
    errors::MarketplaceError, state::{Listing, ListingArgs},
    constants::SEED_LISTING_ACCOUNT,
};

use mpl_core::{
    instructions::{AddPluginV1Cpi, AddPluginV1InstructionArgs},
    types::{PluginAuthority, TransferDelegate, Plugin},
};
use anchor_spl::token_interface::Mint;



#[derive(Accounts)]
#[instruction(args: ListingArgs)]
pub struct CreateListing<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    // Mint can be of the type Token or Token2022
    price_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        space = Listing::INIT_SPACE,
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


impl<'info> CreateListing<'info> {

    // Create a listing to offer a service
    pub fn create_listing(&mut self, args: ListingArgs, bumps: &CreateListingBumps) -> Result<()> {

        self.listing.set_inner(Listing {
            // Address of the seller
            lister: self.payer.key(),
            // Mint in which the price of the service is denominated
            mint: self.price_mint.key(),
            // Selling price (excluding royalties)
            price: args.price.ok_or(MarketplaceError::MissingPrice)?,
            // Seed is a identifier for this listing, unique per seller
            seed: args.seed,
            bump: bumps.listing,

        });

        Ok(())
    }

    // Delegate transfer authority to program so it can transfer the service NFT without requiring seller interaction
    // Alternative to placing NFT in a vault or escrow
    pub fn delegate_transfer_authority(&mut self) -> Result<()> {

        // Create TransferDelegate plugin
        let transfer_delegate_plugin = AddPluginV1InstructionArgs {
            plugin: Plugin::TransferDelegate(TransferDelegate{}),
            // Set authority to the Listing PDA
            init_authority: Some(PluginAuthority::Address { address: self.listing.key() }),
        };

        // CPI into the Metaplex Core program with the add plugin instruction
        AddPluginV1Cpi {
            // Public key of the NFT
            asset: &self.asset.to_account_info(),
            // Collection to which the asset/nft belongs
            collection: self.collection.as_ref(),
            // Authority for authority-managed plugins
            // more info on https://developers.metaplex.com/core/plugins#plugin-table
            authority: self.authority.as_deref(),
            // Payer funds the NFT creation
            payer: &self.payer.to_account_info(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            // Command for the mpl-core program: the TransferDelegate plugin we want to add
            __args: transfer_delegate_plugin,
        }
        .invoke()?;

        Ok(())

    }

}



