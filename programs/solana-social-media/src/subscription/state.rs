use anchor_lang::prelude::*;

#[account]
pub struct Subscription {
    pub subscriber: Pubkey,
    pub creator: Pubkey,        
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Bumps {
    pub post: Pubkey,
}