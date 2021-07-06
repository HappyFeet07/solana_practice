use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::create_account,
    rent::Rent,
    program_pack::{Pack}, program::{invoke },
};

use spl_token::{
    instruction::{initialize_mint, initialize_account, mint_to},
};

entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Create ARF Token");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;

    let mint_account_size = spl_token::state::Mint::LEN;
    let mint_lamports_size = Rent::default().minimum_balance(mint_account_size);

    msg!("Instruction: create account and store it as Mint");
    let create_instruction = create_account(
        payer_account.key,
        mint_account.key,
        mint_lamports_size,
        mint_account_size as u64,
        &spl_token::id(),
    );

    invoke(
        &create_instruction,
        &[
            payer_account.clone(),
            mint_account.clone(),
        ]
    )?;

    let iniMint_inst = initialize_mint(
       &spl_token::id(),
       mint_account.key,
       payer_account.key,
       None,
       10 as u8,
    ).unwrap();

    invoke(
       &iniMint_inst,
       &[
            mint_account.clone(),
            payer_account.clone(),
       ]
    )?;

    let mint_to_account_size = spl_token::state::Account::LEN;
    let mint_to_lamports_size = Rent::default().minimum_balance(mint_to_account_size);
    let mint_to_account = next_account_info(accounts_iter)?;


    msg!("Instruction: mint to");
    let mint_to_inst = create_account(
        payer_account.key,
        mint_to_account.key,
        mint_to_lamports_size,
        mint_to_account_size as u64,
        &spl_token::id(),
    );

    invoke(
        &mint_to_inst,
        &[
            mint_to_account.clone(),
            payer_account.clone(),
        ]
    )?;
    
    msg!("Instruction: Initialize account");
    let initial_account = initialize_account(
        &spl_token::id(),
        mint_to_account.key,
        mint_account.key,
        payer_account.key,
    ).unwrap();

    invoke(
        &initial_account, 
        &[
            mint_to_account.clone(),
            mint_account.clone(),
            payer_account.clone(),
        ]
    )?;

    let mint_to_inst = mint_to(
        &spl_token::id(),
        mint_account.key,
        mint_to_account.key,
        payer_account.key,
        &[],
        10000000000000000,
    ).unwrap();

    invoke(
        &mint_to_inst,
        &[
            mint_account.clone(),
            mint_to_account.clone(),
            payer_account.clone(),
        ]
    )?;

    Ok(())
}
