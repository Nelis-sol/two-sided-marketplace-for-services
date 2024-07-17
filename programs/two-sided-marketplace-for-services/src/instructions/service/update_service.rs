use anchor_lang::prelude::*;
use crate::helpers::{get_accountinfo_option, get_signer_option};
use mpl_core::instructions::{UpdateV1InstructionArgs, UpdateV1Cpi};

#[derive(Accounts)]
pub struct UpdateService<'info> {
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,
    pub authority: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

impl<'info> UpdateService<'info> {

    // Update the NFT to set the name, uri and/or a new update authority
    pub fn update_service(&mut self, args: UpdateV1InstructionArgs) -> Result<()> {

        // Check if accounts do not have the default public key
        // workaround needed to facilitate (Rust) API's that are built with Tokio
        let collection_option = get_accountinfo_option(self.collection.clone());
        let authority_option = get_signer_option(self.authority.clone());
        let log_wrapper_option = get_accountinfo_option(self.log_wrapper.clone());

        // CPI into Metaplex Core program with the update instruction
        UpdateV1Cpi {
            __program: &self.mpl_core,
            // Public key of the NFT
            asset: &self.asset.as_ref(),
            // Collection to which the asset/nft belongs
            collection: collection_option.as_ref(),
            // Payer funds the NFT creation
            payer: &self.payer.to_account_info(),
            // Authority for authority-managed plugins
            // more info on https://developers.metaplex.com/core/plugins#plugin-table
            authority: authority_option.as_deref(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: log_wrapper_option.as_ref(),
            // Commands for the mpl-core program: update the name, uri and/or update authority
            __args: mpl_core::instructions::UpdateV1InstructionArgs {
                new_name: args.new_name,
                new_uri: args.new_uri,
                new_update_authority: args.new_update_authority,
            },
        }
        .invoke()?;

        Ok(())

    }

}
