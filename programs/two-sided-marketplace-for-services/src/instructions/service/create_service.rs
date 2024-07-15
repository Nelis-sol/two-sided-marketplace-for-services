use anchor_lang::prelude::*;
use crate::{
    constants::{ROYALTY_FEE_BPS, ROYALTY_FEE_WALLET},
    errors::MarketplaceError,
};

use mpl_core::{
    instructions::{CreateV1Cpi, CreateV1InstructionArgs},
    types::{DataState, PluginAuthorityPair, Plugin, Royalties, Creator, RuleSet, PermanentFreezeDelegate, PluginAuthority::Address},
};


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

    // Create a NFT that represents a service (e.g. 1 hour coding lesson) that can be bought or (re-)sold on a marketplace
    pub fn create_service(&mut self, args: CreateV1Args) -> Result<()> {

        // List where we add plugins we want to include in the NFT
        let mut plugin_list: Vec<PluginAuthorityPair> = Vec::new();

        // Calculate the percentage of the total royalty (creator + marketplace) that goes to the creator
        let creator_royalty = args.royalty
            .checked_div(ROYALTY_FEE_BPS + args.royalty).ok_or(MarketplaceError::Underflow)?
            .checked_mul(100).ok_or(MarketplaceError::Overflow)? as u8;

        // Prepare the Royalty plug for mpl-core
        let service_royalty = Royalties{
            // Royalty in basispoints
            basis_points: ROYALTY_FEE_BPS + args.royalty,
            creators: vec![
                // Royalty that goes to the creator
                Creator{address: self.payer.key(), percentage: creator_royalty}, 
                // Royalty that goes to the marketplace
                Creator{address: ROYALTY_FEE_WALLET, percentage: 100 - creator_royalty}],
            rule_set: RuleSet::None,
        };

        // Create Royalty plugin
        let royalty_plugin = PluginAuthorityPair {
            plugin: Plugin::Royalties(service_royalty),
            // Set authority to None so the royalties can not be changed after creation
            authority: None,
        };

        // Add Royalty plugin to the plugin list
        plugin_list.push(royalty_plugin);


        // Create Freeze plugin, so the original owner can freeze the service NFT / render them soulbound
        if args.freezable == true {
            let freeze_plugin = PluginAuthorityPair {
                // Start the NFT in unfrozen state (i.e. can be transfered to the buyer)
                plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate{frozen: false}),
                // Set original creator as authority for freezing the NFT
                authority: Some(Address{
                    address: self.payer.key()
                }),
            };

            // Add Freeze plugin to the plugin list
            plugin_list.push(freeze_plugin);
        }


        // CPI into Metaplex Core program with the create instruction
        CreateV1Cpi {
            // Public key of the NFT
            asset: &self.asset.to_account_info(),
            // Collection to which the asset/nft belongs
            collection: self.collection.as_ref(),
            // Authority for authority-managed plugins
            // more info on https://developers.metaplex.com/core/plugins#plugin-table
            authority: self.authority.as_deref(),
            // Payer funds the NFT creation
            payer: &self.payer.to_account_info(),
            // The address to with the NFT is minted
            owner: self.owner.as_ref(),
            // Address that can update the Name and URI
            update_authority: self.update_authority.as_ref(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            // Commands for the mpl-core program: data to add to the NFT and (optional) plugins to add
            __args: CreateV1InstructionArgs {
                data_state: DataState::AccountState,
                name: args.name,
                uri: args.uri,
                plugins: Some(plugin_list),
            },
        }
        .invoke()?;

        Ok(())

    }
}



#[derive(AnchorDeserialize, AnchorSerialize)]
// Struct with information client send to this program to customize the service NFT
pub struct CreateV1Args {
    // Name is stored on-chain
    pub name: String,
    // URI points to a json that conforms to the Metaplex NFT standard, preferably stored on immutable data storage
    // https://developers.metaplex.com/token-metadata/token-standard#the-non-fungible-standard
    pub uri: String,
    // Royalty the creator wishes to receive in basis points
    pub royalty: u16,
    // Creator can opt for the ability to freeze the NFT (e.g. in case of soulbound NFT's)
    pub freezable: bool,
}
