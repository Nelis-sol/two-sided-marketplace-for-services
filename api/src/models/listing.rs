use solana_sdk::pubkey::Pubkey;

pub struct Listing {
    pub lister: Pubkey,  // 32 byte
    pub mint: Pubkey,   // 32 byte
    pub price: u64,     // 8 byte
    pub seed: u64,      // 8 byte
    pub bump: u8,       // 1 byte
}