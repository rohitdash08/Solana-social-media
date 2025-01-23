ues anchor_lang::prelude::*;
use crate::tip::state::Tip;

#[derive(Accounts)]
pub struct SendTip<'info> {
    #[account(init, payer = from, space = 8 + 32 + 32 + 8 + 8)]
    pub tip: Account<'info, Tip>,
    #[account(mut)]
    pub from: Signer<'info>,
    pub to: AccountInfo<'info>
    pub system_program: Program<'info, System>,
}

pub fn send_tip(
    ctx: Context<SendTip>,
    amount: u64,
) -> Result<()> {
    let tip = &mut ctx.accounts.tip;
    tip.from = *ctx.accounts.from.key;
    tip.to = *ctx.accounts.to.key;
    tip.amount = amount;
    tip.timestamp = Clock::get()?.unix_timestamp;

    let from_account = &ctx.accounts.from;
    let to_account = &ctx.accounts.to;
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        from_account.key,
        to_account.key,
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}