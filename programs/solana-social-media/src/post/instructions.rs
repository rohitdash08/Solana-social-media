use anchor_lang::prelude::*;
use crate::post::state::Post;

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = creator, space = 8 + 64 + 32 + 8 + 1)]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub creator: Signer<'info>
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(mut, has_one: creator, close = creator)]
    pub post: Account<'info, Post>,
    pub creator: Signer<'info>,
}

pub fn create_post(
    ctx: Context<CreatePost>,
    content_hash: String,
    is_exclusive: bool,
) -> Result<()> {
    let post = &mut ctx.accounts.post;
    post.content_hash = content_hash;
    post.creator = *ctx.accounts.creator.key;
    post.timestamp = Clock::get()?.unix_timestamp;
    post.is_exclusive = is_exclusive;
    Ok(())
}

pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
    // The post account will be close automatically by Anchor
    Ok(())
}