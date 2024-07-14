use anchor_lang::prelude::*;
use mpl_core::{
    PermanentFreezeDelegatePlugin,
    instructions::{CreateV1Cpi, CreateV1InstructionArgs},
    types::{DataState, PluginAuthorityPair, PluginAuthority, Plugin, Royalties, Creator, RuleSet, PermanentFreezeDelegate},
    advanced_types::{AuthorityType, BaseAuthority, BasePlugin},
};
use mpl_core::types::PluginAuthority::Address;
use crate::constants::ROYALTY_FEE_BPS;


#[derive(Accounts)]
pub struct CreateService<'info> {
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

impl<'info> CreateService<'info> {

    pub fn create_service(&mut self, args: CreateV1Args) -> Result<()> {

        let service_royalty = Royalties{
            basis_points: ROYALTY_FEE_BPS,
            creators: vec![Creator{address: self.payer.key(), percentage: 100}],
            rule_set: RuleSet::None,
        };

        let royalty_plugin = PluginAuthorityPair {
            plugin: Plugin::Royalties(service_royalty),
            authority: None,
        };


        let freeze_plugin = PluginAuthorityPair {
            plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate{frozen: false}),
            authority: Some(Address{
                address: self.payer.key()
            }),
        };


        CreateV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            authority: self.authority.as_deref(),
            payer: &self.payer.to_account_info(),
            owner: self.owner.as_ref(),
            update_authority: self.update_authority.as_ref(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: CreateV1InstructionArgs {
                data_state: DataState::AccountState,
                name: args.name,
                uri: args.uri,
                plugins: Some(vec![royalty_plugin, freeze_plugin]),
            },
        }
        .invoke()?;

        Ok(())

    }
}



#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateV1Args {
    pub name: String,
    pub uri: String,
    // pub plugins: Option<Vec<PluginAuthorityPair>>,
}
