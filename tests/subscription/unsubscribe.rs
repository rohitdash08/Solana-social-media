use anchor_lang::prelude::*;
use solana_social_media::subscription;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_unsubscribe() {
    // Set up the test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_social_media",
        program_id,
        processor!(solana_social_media::processor::process_instruction),
    );

    // Create users
    let subscriber = Keypair::new();
    let creator = Keypair::new();
    let subscription_account = Keypair::new();

    // Add users to the test environment
    program_test.add_account(
        subscriber.pubkey(),
        solana_sdk::account::Account {
            lamports: 1000000000, // 1 SOL
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    program_test.add_account(
        creator.pubkey(),
        solana_sdk::account::Account {
            lamports: 1000000000, // 1 SOL
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    // Start the test
    let mut context = program_test.start_with_context().await;

    // Subscribe to a creator
    let duration = 30 * 24 * 60 * 60; // 30 days in seconds

    let ix = subscription::subscribe(
        program_id,
        subscription_account.pubkey(),
        subscriber.pubkey(),
        creator.pubkey(),
        duration,
    );

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&subscriber.pubkey()),
        &[&subscriber, &subscription_account],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Unsubscribe
    let ix = subscription::unsubscribe(
        program_id,
        subscription_account.pubkey(),
        subscriber.pubkey(),
    );

    let transaction =