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
    instruction::initialize_mint,
};


// Declare and export the program's entrypoint
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
    let token_account = next_account_info(accounts_iter)?;

    let token_account_size = spl_token::state::Mint::LEN;
    let lamports_size = Rent::default().minimum_balance(token_account_size);

    msg!("Instruction");
    let create_instruction = create_account(
        payer_account.key,
        token_account.key,
        lamports_size,
        token_account_size as u64,
        &spl_token::id(),
    );

    invoke(
        &create_instruction,
        &[
            payer_account.clone(),
            token_account.clone(),
        ]
    )?;

    let iniMint_inst = initialize_mint(
       &spl_token::id(),
       token_account.key,
       payer_account.key,
       None,
       10 as u8,
    ).unwrap();

    invoke(
       &iniMint_inst,
       &[
            token_account.clone(),
            payer_account.clone(),
       ]
    )?;

    Ok(())
}

