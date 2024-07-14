use anchor_lang::prelude::*;
use mpl_core::instructions::{BurnV1Cpi, BurnV1InstructionArgs};

#[derive(Accounts)]
pub struct DeleteService<'info> {
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

impl<'info> DeleteService<'info> {

    pub fn delete_service(&mut self, args: BurnV1InstructionArgs) -> Result<()> {

        BurnV1Cpi {
            __program: &self.mpl_core,
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            payer: &self.payer.to_account_info(),
            authority: self.authority.as_deref(),
            system_program: Some(&self.system_program.to_account_info()),
            log_wrapper: self.log_wrapper.as_ref(),
            __args: mpl_core::instructions::BurnV1InstructionArgs {
                compression_proof: None,
            },
        }
        .invoke()?;

    Ok(())

    }

}