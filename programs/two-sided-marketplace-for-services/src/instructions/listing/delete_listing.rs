use anchor_lang::prelude::*;
use crate::{
    state::{Listing, ListingArgs},
    constants::SEED_LISTING_ACCOUNT,
};

use mpl_core::{instructions::{RemovePluginV1Cpi, RemovePluginV1InstructionArgs}, 
    types::PluginType,
};
use anchor_spl::token_interface::Mint;


#[derive(Accounts)]
#[instruction(args: ListingArgs)]
pub struct DeleteListing<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    // Mint can be of the type Token or Token2022
    price_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        // PDA is closed and rent is returned to the signer/payer
        close = payer,
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

impl<'info> DeleteListing<'info> {

    // Withdraw transfer authority from the Listing PDA, as the listing is deleted
    pub fn withdraw_transfer_authority(&mut self, _args: ListingArgs) -> Result<()> {

        // CPI into the Metaplex Core program with the remove plugin instruction
        RemovePluginV1Cpi {
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
            // Commands for the mpl-core program: remove the TransferDelegate plugin
            __args: RemovePluginV1InstructionArgs{
                plugin_type: PluginType::TransferDelegate
            },
        }
        .invoke()?;

        Ok(())
    }

}