use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DeleteServiceArgs {
    pub compression_proof: Option<CompressionProof>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CompressionProof {
    pub owner: Pubkey,
    pub update_authority: String,
    pub name: String,
    pub uri: String,
    pub seq: u64,
    pub plugins: Vec<String>,
}
