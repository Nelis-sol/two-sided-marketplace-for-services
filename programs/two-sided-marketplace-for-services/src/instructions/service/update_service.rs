use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{UpdateV1InstructionArgs, UpdateV1Cpi}
};

#[derive(Accounts)]
pub struct UpdateService<'info> {
    /// The address of the new asset.
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

    pub fn update_service(&mut self, args: UpdateV1InstructionArgs) -> Result<()> {

        UpdateV1Cpi {
            __program: &self.mpl_core,
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            payer: &self.payer.to_account_info(),
            authority: self.authority.as_deref(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
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
