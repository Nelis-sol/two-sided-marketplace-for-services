use anchor_lang::prelude::*;
use mpl_core::instructions::{
    CreateV1InstructionArgs,
    UpdateV1InstructionArgs,
    BurnV1InstructionArgs,
};

mod instructions;
use instructions::*;
mod constants;

mod state;
use state::ListingArgs;

mod errors;


declare_id!("4GVYCsgaxtqHW61wddpKtthbFZqvXhREghgE9C45YV7w");

#[program]
pub mod two_sided_marketplace_for_services {
    use super::*;

    /// 01. Services (description / service agreements)

    pub fn create_service(ctx: Context<CreateService>, args: CreateV1Args) -> Result<()> {
        ctx.accounts.create_service(args)
    }

    pub fn update_service(ctx: Context<UpdateService>, args: UpdateV1InstructionArgs) -> Result<()> {
        ctx.accounts.update_service(args)
    }

    pub fn delete_service(ctx: Context<DeleteService>, args: BurnV1InstructionArgs) -> Result<()> {
        ctx.accounts.delete_service(args)
    }


    /// 02. Listing (list a service)
    
    pub fn create_listing(ctx: Context<CreateListing>, args: ListingArgs) -> Result<()> {
        ctx.accounts.create_listing(args, &ctx.bumps)?;
        ctx.accounts.delegate_transfer_authority()
    }

    pub fn update_listing(ctx: Context<UpdateListing>, args: ListingArgs) -> Result<()> {
        ctx.accounts.update_listing(args)
    }

    pub fn delete_listing(ctx: Context<DeleteListing>, args: ListingArgs) -> Result<()> {
        ctx.accounts.withdraw_transfer_authority(args)
    }


    /// 03. Buying (buy a service)
    
    pub fn buy_now(ctx: Context<BuyNow>, args: ListingArgs) -> Result<()> {
        ctx.accounts.send_payment()?;
        ctx.accounts.send_nft(args)
    }


}

