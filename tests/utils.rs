use solana_sdk::{signature::Keypair, signer::Signer};

pub fn create_user() -> Keypair {
    Keypair::new()
}

pub fn fund_user(user: &Keypair, lamports: u64) -> solana_sdk::account::Account {
    solana_sdk::account::Account {
        lamports,
        data: vec![],
        owner: solana_sdk::system_program::id(),
        executable: false,
        rent_epoch: 0,
    }
}