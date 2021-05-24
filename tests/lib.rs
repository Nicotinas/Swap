use borsh::{BorshDeserialize, BorshSerialize};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use solana_program::clock::Epoch;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
};
use std::mem;

use swap::entrypoint::process_instruction;
use swap::state::AccountProfile;
use swap::instruction::SwapInstruction;
// use swap::processor::Processor;


#[tokio::test]
async fn test_swap() {
    let program_id = Pubkey::new_unique();
    let alice_pub_key = Pubkey::new_unique();
    let bob_pub_key = Pubkey::new_unique();

    // println!("program_id = {:?}", program_id);
    // println!("alice_pub_key = {:?}", alice_pub_key);
    // println!("bob_pub_key = {:?}", bob_pub_key);    

    let mut program_test = ProgramTest::new(
        "swap", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    program_test.add_account(
        alice_pub_key,
        Account {
            lamports: 5,
            data:  vec![0_u8; mem::size_of::<AccountProfile>()],
            owner: program_id,
            ..Account::default()
        },
    );

    // #[derive(BorshSerialize, BorshDeserialize, Debug)]
    // struct CreateAccountProfile {
    //     name: String,
    //     age: u8,
    //     dob: String,
    // }

    // #[derive(BorshSerialize, BorshDeserialize, Debug)]
    // struct InstructionData {
    //     // instruction: u8,
    //     name: String,
    //     age: u8,
    //     dob: String,
    // }
    

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    println!("payer = {:?}", &payer.pubkey());
    println!("recent_blockhash = {:?}", recent_blockhash);

    let alice_account_data =  banks_client.get_account(alice_pub_key).await.expect("get_account").expect("Account not found");

    println!("alice_account_get_account_expect = {:?}", alice_account_data);
    
    // println!("account data = {:?}", &alice_account_data.data);
    let mut a = AccountProfile::unpack(&alice_account_data.data);

    println!("after try from slice = {:?}", a);

    let alice_data =  AccountProfile {        
        name: Some(String::from("shivin")),
        age: 22,
        dob: Some(String::from("15/06/1995")),
    };



    // let mut buf = vec![0_u8];
    let mut buf = alice_data.try_to_vec().unwrap();
    // let mut buf = Vec::new();
    // buf.push(0);
    // let f = alice_data.try_to_vec().unwrap();
    // buf.extend_from_slice(&mut alice_data.try_to_vec().unwrap());
    println!("buf = {:?}", buf);
    println!("lenght of buf = {:?}", buf.len());


    // let mut expected = 

    // let k = AccountProfile::try_from_slice(&f).unwrap();
    // println!("k = {:?}", k);

    // LET 

    // let alice_data = CreateAccountProfile {
    //     name: String::from("shivin"),
    //     age: 22,
    //     dob: String::from("15/06/1995"),
    // };

    // let alice_data = alice_data.try_to_vec().unwrap();
    // println!("alice_data = {:?}", alice_data);
    // let mut instruction_data = vec![0];
    // println!("first instruction_data = {:?}", instruction_data);
    // instruction_data.append(&mut alice_data.try_to_vec().unwrap());
    // let b = alice_data.is_u8();
    // println!("is u8 = {:?}", b);
    // let instruction_data = alice_data;
    // let instruction_data = vec![0_u8; mem::size_of::<AccountProfile>()];
    
    // println!("Second Instruction data = {:?}", instruction_data);

    // let mut transaction = Transaction::new_with_payer(
    //     &[Instruction::new_with_borsh(
    //         program_id,
    //         &SwapInstruction::SetUserProfile {
    //             name: String::from("shivin"),
    //             age: 22,
    //             dob: String::from("15/06/1995"),
    //         } ,
    //         vec![AccountMeta::new(alice_pub_key, false)],
    //     )],
    //     Some(&payer.pubkey()),
    // );

    println!("after buf = {:?}", buf);

    

    let mut transaction = Transaction::new_unsigned(
        &[Instruction::new_with_borsh(
            program_id,
            &buf,
            vec![AccountMeta::new(alice_pub_key, false)],
        )],
    );

    // &[0, String::from("shivin").into_bytes(), 22, String::from("15/16/1995").into_bytes()],


    // println!("transaction = {:?}", transaction);
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    // let mut lamports = 5;
    // let mut data = vec![0; mem::size_of::<AccountProfile>()];
    // let account = AccountInfo::new(
    //     &bob_pub_key,
    //     false,
    //     true,
    //     &mut lamports,
    //     &mut data,
    //     &program_id,
    //     false,
    //     Epoch::default(),
    // );

    // let accounts = vec![account];

    // Processor::process(&program_id, &accounts, &buf)

    /// Verify account has two greetings
    let greeted_account = banks_client
        .get_account(alice_pub_key)
        .await
        .expect("get_account")
        .expect("greeted_account not found");

    println!("data = {:?}", &greeted_account.data)

}