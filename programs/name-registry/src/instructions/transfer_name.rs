use crate::errors::NameRegistryError;
use crate::state::{Record, UserRecord};
use anchor_lang::prelude::*;

pub fn transfer_name(ctx: Context<TransferName>, name: String) -> Result<()> {
    let record = &mut ctx.accounts.record;
    let new_user_record = &mut ctx.accounts.new_user_record;

    require!(
        ctx.accounts.user.key() == record.owner,
        NameRegistryError::UserIsNotOwner
    );

    record.owner = ctx.accounts.new_user.key();
    new_user_record.name = name;
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct TransferName<'info> {
    #[account(
        mut,
        seeds = [b"record", name.as_bytes()],
        bump
    )]
    pub record: Account<'info, Record>,

    #[account(
        mut,
        seeds = [b"user_record", user.key().as_ref()],
        bump,
        close = user
    )]
    pub user_record: Account<'info, UserRecord>,

    #[account(
        init,
        payer = user,
        space = 8 + 64,
        seeds = [b"user_record", new_user.key().as_ref()],
        bump
    )]
    pub new_user_record: Account<'info, UserRecord>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: We only use public key to transfer ownership
    pub new_user: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
