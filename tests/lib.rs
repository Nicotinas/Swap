// #[warn(unused_imports)]
use borsh::BorshDeserialize;
use swap::{process_instruction, GreetingAccount};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    // instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    // signature::Signer,
    // transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_swap() {
    let program_id = Pubkey::new_unique();
    let greeted_pubkey = Pubkey::new_unique();
    let greeted_pubkey2 = Pubkey::new_unique();

    println!("{:?}", program_id);
    println!("{:?}", greeted_pubkey);

    let mut program_test = ProgramTest::new(
        "swap", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    // msg!("{:?}", program_test);

    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );



    let (mut banks_client, key_pair, recent_blockhash) = program_test.start().await;

    println!("{:?}", program_id);
    println!("{:?}", greeted_pubkey);
    // msg!("{:?}", banks_client);
    // msg!("{:?}", payer);
    // msg!("{:?}", recent_blockhash);
    // assert_eq!(1,1);

    // Verify account has zero greetings
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get account")
        .expect("Account requrested does not exist");

    println!("Greeting = {:?}", greeted_account);

    // assert_eq!(
    //     GreetingAccount::try_from_slice(&greeted_account.data)
    //         .unwrap()
    //         .counter,
    //     0
    // );

}