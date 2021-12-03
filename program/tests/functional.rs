#![cfg(feature = "test-bpf")]
use borsh::BorshSerialize;
use helloworld::{
    entrypoint::process_instruction, id, instruction::HelloWorldInstruction, state::GreetingAccount,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

#[tokio::test]
async fn test_helloworld() {
    let program_id = id(); // or create new one
    let greeted_key = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "helloworld", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        greeted_key,
        Account {
            lamports: 5,
            data: GreetingAccount { counter: 0 }.try_to_vec().unwrap(),
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // Verify account has zero greetings
    let greeted_data = banks_client
        .get_account_data_with_borsh::<GreetingAccount>(greeted_key)
        .await
        .unwrap();
    assert_eq!(greeted_data.counter, 0);

    // Greet once
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &HelloWorldInstruction::Increment {},
            vec![AccountMeta::new(greeted_key, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has one greeting
    let greeted_data = banks_client
        .get_account_data_with_borsh::<GreetingAccount>(greeted_key)
        .await
        .unwrap();
    assert_eq!(greeted_data.counter, 1);

    // Greet again
    // if using the same transaction hash, need to sign in next block, otherwise need different signer to make different txhash
    let (last_blockhash, _) = banks_client
        .get_new_blockhash(&last_blockhash)
        .await
        .unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &HelloWorldInstruction::Increment {},
            vec![AccountMeta::new(greeted_key, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has two greetings
    let greeted_data = banks_client
        .get_account_data_with_borsh::<GreetingAccount>(greeted_key)
        .await
        .unwrap();
    assert_eq!(greeted_data.counter, 2);
}
