use anchor_lang::prelude::*;
use crate::post::state::*;

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = creator, space = 8 + 64 + 32 + 8 + 1)]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePost<'info> {
    pub fn bumps(&self) -> Bumps {
        Bumps {
            post: *self.post.to_account_info().key,
        }
    }
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(mut, has_one = creator, close = creator)]
    pub post: Account<'info, Post>,
    pub creator: Signer<'info>,
}

impl<'info> DeletePost<'info> {
    pub fn bumps(&self) -> Bumps {
        Bumps {
            post: *self.post.to_account_info().key,
        }
    }
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

pub fn delete_post(_ctx: Context<DeletePost>) -> Result<()> {
    // The post account will be closed automatically by Anchor
    Ok(())
}