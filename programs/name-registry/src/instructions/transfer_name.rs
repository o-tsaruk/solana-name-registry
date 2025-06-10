use anchor_lang::prelude::*;
use crate::state::NameRecord;

pub fn transfer_name(_ctx: Context<TransferName>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct TransferName<'info> {
    #[account(
        mut,
        seeds = [b"record", user.key().as_ref()],
        bump
    )]
    pub record: Account<'info, NameRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub new_user: Signer<'info>,
}
