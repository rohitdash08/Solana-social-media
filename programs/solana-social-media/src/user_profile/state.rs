use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub username: String,
    pub bio: String,
    pub profile_picture: String,
    pub authority: Pubkey,
}