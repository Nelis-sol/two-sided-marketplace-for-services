use anchor_lang::prelude::*;
use mpl_core::instructions::{
    CreateV1InstructionArgs,
    UpdateV1InstructionArgs,
    BurnV1InstructionArgs,
};

mod instructions;
use instructions::*;

mod state;


declare_id!("4GVYCsgaxtqHW61wddpKtthbFZqvXhREghgE9C45YV7w");

#[program]
pub mod two_sided_marketplace_for_services {
    use super::*;

    /// 01. Services (description / service agreements)

    pub fn create_service(ctx: Context<CreateService>, args: CreateV1InstructionArgs) -> Result<()> {
        ctx.accounts.create_service(args)
    }

    pub fn update_service(ctx: Context<UpdateService>, args: UpdateV1InstructionArgs) -> Result<()> {
        ctx.accounts.update_service(args)
    }

    pub fn delete_service(ctx: Context<DeleteService>, args: BurnV1InstructionArgs) -> Result<()> {
        ctx.accounts.delete_service(args)
    }


    /// 02. Listing (list a service)
    
    pub fn create_listing(ctx: Context<CreateListing>, price: u64, seed: u64) -> Result<()> {
        ctx.accounts.create_listing(price, seed, &ctx.bumps)
    }

    pub fn update_listing(ctx: Context<UpdateListing>, price: u64, seed: u64) -> Result<()> {
        ctx.accounts.update_listing(price, seed)
    }

    pub fn delete_listing(ctx: Context<DeleteListing>, seed: u64) -> Result<()> {
        ctx.accounts.withdraw_nft(seed)
    }


    /// 03. Buying (buy a service)
    
    pub fn buy_now(ctx: Context<BuyNow>, seed: u64) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft(seed)?;
        ctx.accounts.close_mint_vault(seed)
    }


    /// 04. Selling (sell a service)

    pub fn sell_now(ctx: Context<SellNow>) -> Result<()> {
        ctx.accounts.sell_now()
    }


}

