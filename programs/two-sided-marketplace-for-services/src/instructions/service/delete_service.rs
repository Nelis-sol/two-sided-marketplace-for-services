use anchor_lang::prelude::*;
use mpl_core::instructions::{BurnV1Cpi, BurnV1InstructionArgs};

#[derive(Accounts)]
pub struct DeleteService<'info> {
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

impl<'info> DeleteService<'info> {

    // Burn the NFT that represents a service (e.g. 1 hour coding lesson)
    pub fn delete_service(&mut self, args: BurnV1InstructionArgs) -> Result<()> {

        // CPI into the Metaplex Core program with the Burn instruction
        BurnV1Cpi {
            __program: &self.mpl_core,
            // Public key of the NFT
            asset: &self.asset.to_account_info(),
            // Collection to which the asset belongs
            collection: self.collection.as_ref(),
            // Payer funds the NFT burning
            payer: &self.payer.to_account_info(),
            // Authority for authority-managed plugins
            // more info on https://developers.metaplex.com/core/plugins#plugin-table
            authority: self.authority.as_deref(),
            system_program: Some(&self.system_program.to_account_info()),
            log_wrapper: self.log_wrapper.as_ref(),
            // Commands for the mpl-core program
            __args: mpl_core::instructions::BurnV1InstructionArgs {
                compression_proof: args.compression_proof,
            },
        }
        .invoke()?;

    Ok(())

    }

}