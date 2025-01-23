use anchor_lang::prelude::*;

#[account]
pub struct Post {
    pub content_hash: String,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub is_exclusive: bool
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Bumps {
    pub post: Pubkey,
}