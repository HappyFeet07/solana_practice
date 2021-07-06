use::solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_program,
    sysvar,
};
use std::str::FromStr;
use spl_token;
mod utils;

const INIT_PROGRAM_ID: &str = 
    "4WWcBxSuciigshG77EeuWw3RH2wmpmsc2gNjeBBoMNb4";

fn main() {
    let my_keypair = utils::load_config_keypair();
    let my_pubkey = my_keypair.pubkey();

    let new_account_keypair = Keypair::new();
    let new_account_pubkey = new_account_keypair.pubkey();
    let new_second_keypair = Keypair::new();
    let new_second_pubkey = new_second_keypair.pubkey();
    let new_third_keypair = Keypair::new();
    let new_third_pubkey = new_third_keypair.pubkey();

    let my_program_id = Pubkey::from_str(INIT_PROGRAM_ID).unwrap();
    println!("{}", sysvar::rent::id());
    println!("My pubkey : {}", my_pubkey);
    println!("My new account : {}", new_account_pubkey);
    let test_ix = Instruction::new(
        my_program_id,
        &(),
        vec![
            AccountMeta::new(my_pubkey, true),
            AccountMeta::new(new_account_pubkey, true),
            AccountMeta::new(new_second_pubkey, true),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
    );

    println!("Test1");

    let rpc_client = utils::new_rpc_client();
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .expect("failed to get recent blockhash");

    println!("Test2");

    let tx = Transaction::new(
        &[
            &my_keypair, 
            &new_account_keypair, 
            &new_second_keypair,
        ],
        Message::new(
            &[test_ix], 
            Some(&my_pubkey)
        ),
        recent_blockhash,
    );

    rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("tx failed");
    println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
