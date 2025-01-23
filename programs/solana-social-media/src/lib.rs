use anchor_lang::prelude::*;

// Replace with actual program ID
declare_id!("DTmB251A85c6vD7HdrmULfPnVuj7AXkRx528kdVaiR1Z");

pub mod user_profile;
pub mod post;
pub mod tip;
pub mod subscription;
pub mod error;

pub use user_profile::*;
pub use post::*;
pub use tip::*;
pub use subscription::*;
pub use error::*;


#[program]
pub mod solana_social_media {
    use super::*;

    // User Profile Instructions
    pub fn create_profile(
        ctx: Context<user_profile::CreateProfile>,
        username: String,
        bio: String,
        profile_picture: String,
    ) -> Result<()> {
        user_profile::create_profile(ctx, username, bio, profile_picture)
    }

    pub fn update_profile(
        ctx: Context<user_profile::UpdateProfile>
        username: String,
        bio: String,
        profile_picture: String,
    ) -> Result<()> {
        user_profile::UpdateProfile(ctx, username, bio, profile_picture)
    }

    // Post Instruction
    pub fn create_post(
        ctx: Context<post::CreatePost>,
        content_hash: String,
        is_exclusive: bool,
    ) -> Result<()> {
        post::create_post(ctx, content_hash, is_exclusive)
    }

    pub fn delete_post(ctx: Context<post::DeletePost>) -> Result<()> {
        post::delete_post(ctx)
    }

    // Tip Instructions
    pub fn send_tip(
        ctx: Context<tip::SendTip>,
        amount: u64,
    ) -> Result<()> {
        tip::send_tip(ctx, amount)
    }

    // Subscription Instructions
    pub fn subscribe(
        ctx: Context<subscription::Subscribe>,
        duration: i64,
    ) -> Result<()> {
        subscription::subscribe(ctx, duration)
    }

    pub fn unsubscribe(ctx: Context<subscription::Unsubscribe>) -> Result<()> {
        subscription::unsubscribe(ctx)
    } 
}

// #[derive(Accounts)]
// pub struct Initialize {}
