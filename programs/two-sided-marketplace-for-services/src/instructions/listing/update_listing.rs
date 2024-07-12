use anchor_lang::prelude::*;
use crate::state::Listing;

use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface}, 
    associated_token::AssociatedToken
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct UpdateListing<'info> {
    #[account(mut)]
    lister: Signer<'info>,
    lister_mint: InterfaceAccount<'info, Mint>,
    collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::authority = lister,
        associated_token::mint = lister_mint,
    )]
    lister_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = lister,
        associated_token::mint = lister_mint,
        associated_token::authority = listing,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [lister_mint.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    listing: Account<'info, Listing>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> UpdateListing<'info> {

    pub fn update_listing(&mut self, price: u64, _seed: u64) -> Result<()> {

        self.listing.price = price;

        Ok(())
    }

}