use anchor_lang::prelude::*;
use crate::subscription::state::Subscription;

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(init, payer = subscriber, space = 8 + 32 + 32 + 8 + 8)]
    pub subscription: Account<'info, Subscription>,
    #[account(mut)]
    pub subscriber: Signer<'info>,
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unsubscribe<'info> {
    #[account(mut, has_one = subscriber, close = subscriber)]
    pub subscription: Account<'info, Subscription>,
    pub subscriber: Signer<'info>,
}

pub fn subscribe(
    ctx: Context<Subscribe>,
    duration: i64, // Duration in seconds
) -> Result<()> {
    let subscription = &mut ctx.accounts.subscription;
    subscription.subscriber = *ctx.accounts.subscriber.key;
    subscription.creator = *ctx.accounts.creator.key;
    subscription.start_time = Clock::get()?.unix_timestamp;
    subscription.end_time = subscription.start_time + duration;
    Ok(())
}

pub fn unsubscribe(ctx: Context<Unsubscribe>) -> Result<()> {
    // The subscription account will be closed automatically by Anchor
    Ok(())
}