use anchor_lang::prelude::*;

#[account]
pub struct Tip {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}