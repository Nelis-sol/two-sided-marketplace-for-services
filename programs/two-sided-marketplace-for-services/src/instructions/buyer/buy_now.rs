use anchor_lang::prelude::*;
use crate::state::{Listing, ListingArgs};
use crate::constants::SEED_LISTING_ACCOUNT;
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked},
    associated_token::AssociatedToken,
};
use mpl_core::{
    instructions::{TransferV1Cpi, TransferV1InstructionArgs, RevokePluginAuthorityV1Cpi, RevokePluginAuthorityV1InstructionArgs},
    types::PluginType
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

    pub fn send_payment(&self) -> Result<()> {

        let accounts = TransferChecked {
            from: self.buyer_ata.to_account_info(),
            to: self.lister_ata.to_account_info(),
            authority: self.buyer.to_account_info(),
            mint: self.price_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            accounts,
        );

        transfer_checked(cpi_ctx, self.listing.price, self.price_mint.decimals)

    }

    pub fn send_nft(&mut self, args: ListingArgs) -> Result<()> {

        let seeds = &[
            SEED_LISTING_ACCOUNT, 
            &self.lister.key().to_bytes(), 
            &self.listing.seed.to_le_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];


       TransferV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            payer: &self.buyer.to_account_info(),
            authority: self.authority.as_ref(),
            new_owner: &self.buyer.to_account_info(),
            system_program: Some(self.system_program.as_ref()),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: TransferV1InstructionArgs {
                compression_proof: None,
            },
        }.invoke_signed(signer_seeds)?;


        RevokePluginAuthorityV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            authority: None,
            payer: &self.buyer.to_account_info(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: RevokePluginAuthorityV1InstructionArgs {
                plugin_type: PluginType::TransferDelegate,
            },
        }.invoke()?;


        Ok(())

    }


}