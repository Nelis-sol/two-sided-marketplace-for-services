use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UpdateV1InstructionArgs {
    pub new_name: Option<String>,
    pub new_uri: Option<String>,
    pub new_update_authority: Option<UpdateAuthority>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum UpdateAuthority {
    None,
    Address(Pubkey),
    Collection(Pubkey),
}