use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct SellNow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> SellNow<'info> {

    pub fn sell_now(&mut self) -> Result<()> {
        Ok(())
    }

}