use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateService<'info> {

    pub fn update_service(&mut self) -> Result<()> {
        Ok(())
    }

}