use axum::{
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use dotenv::dotenv;
use std::{
    env,
    str::FromStr,
};

use crate::{
    models::create_service::CreateServiceArgs,
    constants::*
};

use solana_sdk::{
    instruction::AccountMeta, message::Message, pubkey::Pubkey, signer::Signer, system_program,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::{Keypair, read_keypair_file};



pub async fn create_service(
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("could not find RPC_URL");

    // Extract values from json input
    let name_input: String = payload.get("name").expect("Could not find name in payload").to_string();
    let uri_input: String = payload.get("uri").expect("Could not find uri in payload").to_string();
    let royalty_input: u16 = payload.get("royalty").expect("Could not find royalty in payload").as_u64().expect("Could not convert royalty to u64") as u16;
    let freezable_input: bool = payload.get("freezable").expect("Could not find uri in payload").as_bool().expect("Could not convert freezable to boolean");
    
    // Construct keypairs: signer, asset and default_keypair
    let signer = read_keypair_file("keypair.json").expect("Could not read keypair from keypair.json");
    let asset = Keypair::new();
    // default_keypair is a workaround because solana_sdk can not handle optional values expected by Anchor
    let default_keypair = read_keypair_file("default_keypair.json").expect("Could not read default_keypair from default_keypair.json");
    let default_keypair_publickey = default_keypair.pubkey();

    // Initiate the RPC client to send transactions to the Solana network
    let connection = RpcClient::new(String::from(&rpc_url));

    // Retrieve the system program and metaplex core program id
    let program_id = Pubkey::from_str(PROGRAM_ID_STR).expect("Could not create Pubkey from PROGRAM_ID_STR");
    let metaplex_core_program_id = Pubkey::from_str(METAPLEX_CORE_PROGRAM_ID_STR).expect("Could not create Pubkey from constant");


    // Create arguments needed for the anchor function we are calling
    let service_args = CreateServiceArgs {
        name: name_input,
        uri: uri_input,
        royalty: royalty_input,
        freezable: freezable_input,
    };

    // Create instruction
    let ix = solana_sdk::instruction::Instruction::new_with_borsh(
        program_id,
        // Anchor works with discriminants to identify the function that's being called
        // in this case we are calling the CreateService function and sending the listing_args as arguments
        &(CREATE_SERVICE_DISCRIMINANT, service_args),
        // Add accounts in the order Anchor expects them (incl. mutability and writeability)
        vec![
            AccountMeta::new(asset.pubkey(), true),
            AccountMeta::new(default_keypair_publickey, true),
            AccountMeta::new(default_keypair_publickey, true),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new(default_keypair_publickey, true),
            AccountMeta::new(default_keypair_publickey, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(default_keypair_publickey, false),
            AccountMeta::new_readonly(metaplex_core_program_id, false),
        ],
    );

    // Construct a message by adding the instruction and a payer for the transaction together
    let message = Message::new(&[ix], Some(&signer.pubkey()));

    // Construct (unsigned) transaction
    let mut tx = Transaction::new_unsigned(message);

    // Sign transaction with the appropriate keypairs
    tx.sign(&[&signer, &asset, &default_keypair], connection.get_latest_blockhash().unwrap());

    // Blast of transaction to the RPC provider
    let tx_id = connection
        .send_and_confirm_transaction_with_spinner(&tx)
        .map_err(|err| {
            println!("{:?}", err);
        })
        .unwrap();

    println!("tx: {}", tx_id.to_string());

    // Respond to client with the signature of the transaction
    Json(tx_id.to_string()).into_response()
}