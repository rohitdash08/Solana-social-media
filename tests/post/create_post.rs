use anchor_lang::prelude::*;
use solana_social_media::post;
use solana_social_media::state::Post;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_create_post() {
    // Set up the test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_social_media",
        program_id,
        processor!(solana_social_media::processor::process_instruction),
    );

    // Create a user
    let user = Keypair::new();
    let post_account = Keypair::new();

    // Add the user to the test environment
    program_test.add_account(
        user.pubkey(),
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

    // Create a post
    let content_hash = "QmExampleContentHash".to_string();
    let is_exclusive = false;

    let ix = post::create_post(
        program_id,
        post_account.pubkey(),
        user.pubkey(),
        content_hash.clone(),
        is_exclusive,
    );

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&user.pubkey()),
        &[&user, &post_account],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Fetch the created post
    let post_account_data = context
        .banks_client
        .get_account(post_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let post_data = Post::try_from_slice(&post_account_data.data).unwrap();

    // Verify the post data
    assert_eq!(post_data.content_hash, content_hash);
    assert_eq!(post_data.creator, user.pubkey());
    assert_eq!(post_data.is_exclusive, is_exclusive);
}