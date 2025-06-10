use crate::errors::NameRegistryError;
use crate::state::NameRecord;
use anchor_lang::prelude::*;

pub fn transfer_name(ctx: Context<TransferName>, _name: String) -> Result<()> {
    let record = &mut ctx.accounts.record;
    require!(
        ctx.accounts.user.key() == record.owner,
        NameRegistryError::UserIsNotOwner
    );
    record.owner = ctx.accounts.new_user.key();
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
    pub record: Account<'info, NameRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: We only use public key to transfer ownership
    pub new_user: UncheckedAccount<'info>,
}
