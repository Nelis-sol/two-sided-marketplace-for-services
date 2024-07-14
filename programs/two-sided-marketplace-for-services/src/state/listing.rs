use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub lister: Pubkey,  // 32 byte
    pub mint: Pubkey,   // 32 byte
    pub price: u64,     // 8 byte
    pub seed: u64,      // 8 byte
    pub bump: u8,       // 1 byte
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 8 + 1;
}


#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ListingArgs {
    pub seed: u64,
    pub price: Option<u64>,
}