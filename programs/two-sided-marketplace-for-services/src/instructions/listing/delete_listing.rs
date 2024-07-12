use anchor_lang::prelude::*;
use crate::state::Listing;

use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked}, 
    associated_token::AssociatedToken
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct DeleteListing<'info> {
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
        close=lister,
        seeds = [lister_mint.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    listing: Account<'info, Listing>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> DeleteListing<'info> {

    pub fn withdraw_nft(&mut self, seed: u64) -> Result<()> {
        let seeds = &[
            &self.lister_mint.key().to_bytes()[..],
            &seed.to_le_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.lister_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.lister_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(cpi_ctx, 1, self.lister_mint.decimals)
    }

}