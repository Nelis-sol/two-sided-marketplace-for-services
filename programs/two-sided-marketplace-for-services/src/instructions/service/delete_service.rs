use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DeleteService<'info> {

    pub fn delete_service(&mut self) -> Result<()> {
        Ok(())
    }

}