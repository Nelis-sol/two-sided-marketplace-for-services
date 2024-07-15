use anchor_lang::prelude::*;
use mpl_core::instructions::{UpdateV1InstructionArgs, UpdateV1Cpi};

#[derive(Accounts)]
pub struct UpdateService<'info> {
    /// The address of the asset.
    #[account(mut)]
    pub asset: Signer<'info>,
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,
    pub authority: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Checked in mpl-core.
    pub owner: Option<AccountInfo<'info>>,
    /// CHECK: Checked in mpl-core.
    pub update_authority: Option<AccountInfo<'info>>,
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

        // CPI into Metaplex Core program with the update instruction
        UpdateV1Cpi {
            __program: &self.mpl_core,
            // Public key of the NFT
            asset: &self.asset.to_account_info(),
            // Collection to which the asset/nft belongs
            collection: self.collection.as_ref(),
            // Payer funds the NFT creation
            payer: &self.payer.to_account_info(),
            // Authority for authority-managed plugins
            // more info on https://developers.metaplex.com/core/plugins#plugin-table
            authority: self.authority.as_deref(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
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
