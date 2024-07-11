use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

mod state;


declare_id!("EQY5RoA9unEGkvY8DQVK5v7qq1AMHKSKNQmrkQ39wmFb");

#[program]
pub mod two_sided_marketplace_for_services {
    use super::*;

    /// 01. Services (description / service agreements)

    pub fn create_service(ctx: Context<CreateService>) -> Result<()> {
        ctx.accounts.create_service()
    }

    pub fn update_service(ctx: Context<UpdateService>) -> Result<()> {
        ctx.accounts.update_service()
    }

    pub fn delete_service(ctx: Context<DeleteService>) -> Result<()> {
        ctx.accounts.delete_service()
    }


    /// 02. Selling (listing a service)
    
    pub fn create_listing(ctx: Context<CreateListing>) -> Result<()> {
        ctx.accounts.create_listing()
    }

    pub fn update_listing(ctx: Context<UpdateListing>) -> Result<()> {
        ctx.accounts.update_listing()
    }

    pub fn delete_listing(ctx: Context<DeleteListing>) -> Result<()> {
        ctx.accounts.delete_listing()
    }


    /// 03. Buying (buy a service)
    
    pub fn buy_now(ctx: Context<BuyNow>) -> Result<()> {
        ctx.accounts.buy_now()
    }


}

