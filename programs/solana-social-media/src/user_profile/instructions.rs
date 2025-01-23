use anchor_lang::prelude::*;
use crate::user_profile::state::UserProfile;

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 200 + 64 + 32)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut, has_one = authority)]
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}

pub fn create_profile(
    ctx: Context<CreateProfile>,
    username: String,
    bio: String,
    profile_picture: String,
) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.username = username;
    user_profile.bio = bio;
    user_profile.profile_picture = profile_picture;
    user_profile.authority = *ctx.accounts.authority.key;
    Ok(())
}

pub fn update_profile(
    ctx: Context<UpdateProfile>,
    username: String,
    bio: String,
    profile_picture: String,
) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.username = username;
    user_profile.bio = bio;
    user_profile.profile_picture = profile_picture;
    Ok(())
}