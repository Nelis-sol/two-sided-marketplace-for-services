use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CreateServiceArgs {
    pub name: String,
    pub uri: String,
    pub royalty: u16,
    pub freezable: bool,
}