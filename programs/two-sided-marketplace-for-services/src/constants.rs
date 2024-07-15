use anchor_lang::{pubkey, solana_program::pubkey::Pubkey};

pub const SEED_LISTING_ACCOUNT: &[u8] = b"listing";
pub const ROYALTY_FEE_BPS: u16 = 100;
pub const ROYALTY_FEE_WALLET: Pubkey = pubkey!("2wy3g8KC8QQz92TyEhAxP63WZEdu4uRfnj58DRQmx2bn");