use anchor_lang::prelude::*;
use crate::{
    constants::SEED_LISTING_ACCOUNT,
    state::{Listing, ListingArgs},
};

use mpl_core::{
    instructions::{TransferV1Cpi, TransferV1InstructionArgs, RevokePluginAuthorityV1Cpi, RevokePluginAuthorityV1InstructionArgs},
    types::PluginType
};
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked},
    associated_token::AssociatedToken,
};


#[derive(Accounts)]
#[instruction(args: ListingArgs)]
pub struct BuyNow<'info> {
    #[account(mut)]
    buyer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = price_mint,
        associated_token::authority = buyer,
    )]
    buyer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    lister: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = price_mint,
        associated_token::authority = lister,
    )]
    lister_ata: InterfaceAccount<'info, TokenAccount>,
    price_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        close = lister,
        seeds = [SEED_LISTING_ACCOUNT, lister.key().as_ref(), args.seed.to_le_bytes().as_ref()],
        bump
    )]
    listing: Account<'info, Listing>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,
    /// The owner or delegate of the asset.
    // CHECK: Checked in mpl-core
    pub authority: Option<AccountInfo<'info>>,
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

impl<'info> BuyNow<'info> {

    // Pay for the service (NFT)
    pub fn send_payment(&self) -> Result<()> {

        // Load the accounts involved in the payment for the service (NFT)
        // Payment happens over token accounts, either a SPL token (e.g. USDC) or Wrapped SOL
        let accounts = TransferChecked {
            from: self.buyer_ata.to_account_info(),
            to: self.lister_ata.to_account_info(),
            authority: self.buyer.to_account_info(),
            mint: self.price_mint.to_account_info(),
        };

        // Create the CPI context using the accounts and in our case the token program
        // The token program is an InterfaceAccount meaning it can be both the 'old' Token and the 'new' Token2022 program
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            accounts,
        );

        // Transfer the payment which is equal to the listing price
        // Royalty payments are handled by the mpl-core program
        transfer_checked(cpi_ctx, self.listing.price, self.price_mint.decimals)

    }

    // Send the NFT to the buyer
    pub fn send_nft(&mut self, _args: ListingArgs) -> Result<()> {

        // Derive the signer seeds of the Listing PDA
        // Listing PDA has transfer authority over the NFT so will need to sign for the transfer
        let seeds = &[
            SEED_LISTING_ACCOUNT, 
            &self.lister.key().to_bytes(), 
            &self.listing.seed.to_le_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];


        // CPI into the Metaplex Core program with the transfer instruction
        TransferV1Cpi {
            // Public key of the NFT
            asset: &self.asset.to_account_info(),
            // Collection to which the asset/nft belongs
            collection: self.collection.as_ref(),
            // Payer funds the NFT creation
            payer: &self.buyer.to_account_info(),
            // Authority for authority-managed plugins
            // more info on https://developers.metaplex.com/core/plugins#plugin-table
            authority: self.authority.as_ref(),
            // Address where the NFT will be transfered to
            new_owner: &self.buyer.to_account_info(),
            system_program: Some(self.system_program.as_ref()),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            // Commands for the mpl-core program: an optional compression proof
            __args: TransferV1InstructionArgs {
                compression_proof: None,
            },
            // use signer seeds for invoking the CPI so the runtime will recognize our PDA as a signer
        }.invoke_signed(signer_seeds)?;


        // CPI into the Metaplex Core program with the revoke plugin authority instruction
        // The Listing PDA should no longer be allowed to transfer the NFT
        RevokePluginAuthorityV1Cpi {
            // Public key of the NFT
            asset: &self.asset.to_account_info(),
            // Collection to which the asset/nft belongs
            collection: self.collection.as_ref(),
            // Authority for authority-managed plugins - in this case None as we remove the Listing PDA as authority
            // more info on https://developers.metaplex.com/core/plugins#plugin-table
            authority: None,
            // Payer funds the transaction
            payer: &self.buyer.to_account_info(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            // Commands for the mpl-core program: remove the TransferDelegate plugin
            __args: RevokePluginAuthorityV1InstructionArgs {
                plugin_type: PluginType::TransferDelegate,
            },
        }.invoke()?;


        Ok(())

    }


}