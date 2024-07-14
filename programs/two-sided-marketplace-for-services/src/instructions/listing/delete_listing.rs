use anchor_lang::prelude::*;
use crate::state::{Listing, ListingArgs};
use anchor_spl::token_interface::Mint;
use mpl_core::{instructions::{RemovePluginV1Cpi, RemovePluginV1InstructionArgs}, 
    types::{TransferDelegate, PluginType},
};
use crate::constants::SEED_LISTING_ACCOUNT;


#[derive(Accounts)]
#[instruction(args: ListingArgs)]
pub struct DeleteListing<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    price_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        close = payer,
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

    pub fn withdraw_transfer_authority(&mut self, _args: ListingArgs) -> Result<()> {

        RemovePluginV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            authority: self.authority.as_deref(),
            payer: &self.payer.to_account_info(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: RemovePluginV1InstructionArgs{
                plugin_type: PluginType::TransferDelegate
            },
        }
        .invoke()?;

        Ok(())
    }

}