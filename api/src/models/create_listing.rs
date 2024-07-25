use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ListingArgs {
    pub seed: u64,
    pub price: Option<u64>,
}