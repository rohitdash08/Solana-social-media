use anchor_lang::prelude::*;
use solana_social_media::tip;
use solana_social_media::state::Tip;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_send_tip() {
    // Set up the test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_social_media",
        program_id,
        processor!(solana_social_media::processor::process_instruction),
    );

    // Create users
    let tipper = Keypair::new();
    let creator = Keypair::new();
    let tip_account = Keypair::new();

    // Add users to the test environment
    program_test.add_account(
        tipper.pubkey(),
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

    // Send a tip
    let amount = 1000000; // 0.001 SOL

    let ix = tip::send_tip(
        program_id,
        tip_account.pubkey(),
        tipper.pubkey(),
        creator.pubkey(),
        amount,
    );

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&tipper.pubkey()),
        &[&tipper, &tip_account],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Fetch the tip account
    let tip_account_data = context
        .banks_client
        .get_account(tip_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let tip_data = Tip::try_from_slice(&tip_account_data.data).unwrap();

    // Verify the tip data
    assert_eq!(tip_data.from, tipper.pubkey());
    assert_eq!(tip_data.to, creator.pubkey());
    assert_eq!(tip_data.amount, amount);
}