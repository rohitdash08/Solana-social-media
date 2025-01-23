use anchor_lang::prelude::*;

// Declare the program ID (replace with your actual program ID)
declare_id!("DTmB251A85c6vD7HdrmULfPnVuj7AXkRx528kdVaiR1Z");

// Import modules for each program
pub mod error;
pub mod post;
pub mod subscription;
pub mod tip;
pub mod user_profile;

// Re-export only the necessary items
use crate::{
    post::instructions::*,
    subscription::instructions::*,
    tip::instructions::*,
    user_profile::instructions::*,
};

#[program]
pub mod solana_social_media {
    use super::*;

    // User Profile Instructions
    pub fn create_profile(
        ctx: Context<CreateProfile>,
        username: String,
        bio: String,
        profile_picture: String,
    ) -> Result<()> {
        user_profile::instructions::create_profile(ctx, username, bio, profile_picture)
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        username: String,
        bio: String,
        profile_picture: String,
    ) -> Result<()> {
        user_profile::instructions::update_profile(ctx, username, bio, profile_picture)
    }

    // Post Instructions
    pub fn create_post(
        ctx: Context<CreatePost>,
        content_hash: String,
        is_exclusive: bool,
    ) -> Result<()> {
        post::instructions::create_post(ctx, content_hash, is_exclusive)
    }

    pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
        post::instructions::delete_post(ctx)
    }

    // Tip Instructions
    pub fn send_tip(
        ctx: Context<SendTip>,
        amount: u64,
    ) -> Result<()> {
        tip::instructions::send_tip(ctx, amount)
    }

    // Subscription Instructions
    pub fn subscribe(
        ctx: Context<Subscribe>,
        duration: i64, // Duration in seconds
    ) -> Result<()> {
        subscription::instructions::subscribe(ctx, duration)
    }

    pub fn unsubscribe(ctx: Context<Unsubscribe>) -> Result<()> {
        subscription::instructions::unsubscribe(ctx)
    }
}