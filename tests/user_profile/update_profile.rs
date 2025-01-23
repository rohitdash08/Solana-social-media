use anchor_lang::prelude::*;
use solana_social_media::user_profile;
use solana_social_media::state::UserProfile;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_update_profile() {
    // Set up the test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_social_media",
        program_id,
        processor!(solana_social_media::processor::process_instruction),
    );

    // Create a user
    let user = Keypair::new();
    let user_profile_account = Keypair::new();

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

    // Create a profile
    let username = "test_user".to_string();
    let bio = "This is a test bio.".to_string();
    let profile_picture = "https://example.com/profile.jpg".to_string();

    let ix = user_profile::create_profile(
        program_id,
        user_profile_account.pubkey(),
        user.pubkey(),
        username.clone(),
        bio.clone(),
        profile_picture.clone(),
    );

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&user.pubkey()),
        &[&user, &user_profile_account],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Update the profile
    let new_username = "updated_user".to_string();
    let new_bio = "This is an updated bio.".to_string();
    let new_profile_picture = "https://example.com/new_profile.jpg".to_string();

    let ix = user_profile::update_profile(
        program_id,
        user_profile_account.pubkey(),
        user.pubkey(),
        new_username.clone(),
        new_bio.clone(),
        new_profile_picture.clone(),
    );

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&user.pubkey()),
        &[&user],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Fetch the updated profile
    let profile_account = context
        .banks_client
        .get_account(user_profile_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let profile_data = UserProfile::try_from_slice(&profile_account.data).unwrap();

    // Verify the updated profile data
    assert_eq!(profile_data.username, new_username);
    assert_eq!(profile_data.bio, new_bio);
    assert_eq!(profile_data.profile_picture, new_profile_picture);
}