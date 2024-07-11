use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct BuyNow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyNow<'info> {

    pub fn buy_now(&mut self) -> Result<()> {
        Ok(())
    }

}