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
    models::create_listing::ListingArgs,
    constants::*
};

use solana_sdk::{
    instruction::AccountMeta, message::Message, pubkey::Pubkey, signer::Signer, system_program,
    transaction::Transaction,
    signer::keypair::read_keypair_file,
};
use solana_client::rpc_client::RpcClient;
use spl_associated_token_account::get_associated_token_address;



pub async fn buy_now(
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("could not find RPC_URL");

    // Extract values from json input
    let asset_str: String = payload.get("asset").expect("Could not find name in payload").to_string().replace("\"", "");
    println!("asset str: {}", &asset_str);
    let asset = Pubkey::from_str(&asset_str).expect("Couldn't get pubkey from asset string");

    let seed_input: u64 = payload.get("seed").expect("Could not find seed in payload").as_u64().expect("Could not convert seed to u64");
    let price_mint_str: String = payload.get("price_mint").expect("Could not find price_mint in payload").to_string().replace("\"", "");
    let price_mint_input: Pubkey = Pubkey::from_str(&price_mint_str).expect("Could not convert price_mint string to pubkey");

    let mut price_input: Option<u64> = None;
    match payload.get("price") {
        Some(price_option) => { 
            price_input = price_option.as_u64()
        },
        _ => ()
    }

    let token_program_str: String = payload.get("token_program").expect("Could not find token_program in payload").to_string().replace("\"", "");
    let token_program_input: Pubkey = Pubkey::from_str(&token_program_str).expect("Could not convert token_program string to pubkey");

    let lister_str: String = payload.get("lister").expect("Could not find lister in payload").to_string().replace("\"", "");
    let lister_input: Pubkey = Pubkey::from_str(&lister_str).expect("Could not convert lister string to pubkey");


    // Construct keypairs: signer, asset and default_keypair
    let signer = read_keypair_file("keypair.json").expect("Could not read keypair from keypair.json");
    let signer_pubkey: Pubkey = signer.pubkey();

    // default_keypair is a workaround because solana_sdk can not handle optional values expected by Anchor
    let default_keypair = read_keypair_file("default_keypair.json").expect("Could not read default_keypair from default_keypair.json");
    let default_keypair_publickey = default_keypair.pubkey();

    // Initiate the RPC client to send transactions to the Solana network
    let connection = RpcClient::new(String::from(&rpc_url));

    // Retrieve the system program and metaplex core program id
    let program_id = Pubkey::from_str(PROGRAM_ID_STR).expect("Could not create Pubkey from PROGRAM_ID_STR");
    let metaplex_core_program_id = Pubkey::from_str(METAPLEX_CORE_PROGRAM_ID_STR).expect("Could not create Pubkey from constant");

    let seed_bytes = seed_input.to_le_bytes();
    let seeds: &[&[u8]; 3] = &[b"listing", &signer_pubkey.to_bytes(), seed_bytes.as_ref()];
    let (listing_pda, _) = Pubkey::find_program_address(seeds, &program_id);


    println!("default public key: {}", &default_keypair_publickey);

    let buyer_ata = get_associated_token_address(
       &default_keypair_publickey, // buyer surrogate
       &price_mint_input, // token mint
    );

    println!("buyer ata: {}", buyer_ata.to_string());

    let lister_ata = get_associated_token_address(
        &lister_input,
        &price_mint_input, // token mint
     );


    // Create arguments needed for the anchor function we are calling
    let listing_args = ListingArgs {
        seed: seed_input,
        price: price_input,
    };

    let noop_program_id = Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").expect("Could not construct noop pubkey from string");

    // Create instruction
    let ix = solana_sdk::instruction::Instruction::new_with_borsh(
        program_id,
        // Anchor works with discriminants to identify the function that's being called
        // in this case we are calling the CreateService function and sending the listing_args as arguments
        &(BUY_NOW_DISCRIMINANT, listing_args),
        // Add accounts in the order Anchor expects them (incl. mutability and writeability)
        vec![
            AccountMeta::new(default_keypair.pubkey(), true),
            AccountMeta::new(buyer_ata, false),
            AccountMeta::new(lister_input, false),
            AccountMeta::new(lister_ata, false),
            AccountMeta::new_readonly(price_mint_input, false),
            AccountMeta::new(listing_pda, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(token_program_input, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(asset, false),
            AccountMeta::new(default_keypair_publickey, false),           
            AccountMeta::new_readonly(listing_pda, false),
            AccountMeta::new_readonly(default_keypair_publickey, false),
            AccountMeta::new_readonly(metaplex_core_program_id, false),
        ],
    );

    // Construct a message by adding the instruction and a payer for the transaction together
    let message = Message::new(&[ix], Some(&default_keypair.pubkey()));

    // Construct (unsigned) transaction
    let mut tx = Transaction::new_unsigned(message);

    // Sign transaction with the appropriate keypairs
    tx.sign(&[&default_keypair], connection.get_latest_blockhash().unwrap());

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