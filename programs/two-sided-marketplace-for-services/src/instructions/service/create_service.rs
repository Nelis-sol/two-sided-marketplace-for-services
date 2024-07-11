use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateService<'info> {

    pub fn create_service(&mut self) -> Result<()> {
        Ok(())
    }

}