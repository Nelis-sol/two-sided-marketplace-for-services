use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::Listing;

use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account},
    associated_token::AssociatedToken
};


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct BuyNow<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    #[account(mut)]
    lister: SystemAccount<'info>,
    lister_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = lister_mint,
        associated_token::authority = taker,
    )]
    taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = lister_mint,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = lister,
        seeds = [lister_mint.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = listing.bump,
    )]
    listing: Account<'info, Listing>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> BuyNow<'info> {

    pub fn send_sol(&self) -> Result<()> {
        let accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.lister.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi_ctx, self.listing.price)

    }

    pub fn send_nft(&mut self, seed: u64) -> Result<()> {

        let seeds = &[
            &self.lister_mint.key().to_bytes()[..],
            &seed.to_le_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
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

    pub fn close_mint_vault(&mut self, seed: u64) -> Result<()> {
        let seeds = &[
            &self.lister_mint.key().to_bytes()[..],
            &seed.to_le_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.lister.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        close_account(cpi_ctx)
    }

}