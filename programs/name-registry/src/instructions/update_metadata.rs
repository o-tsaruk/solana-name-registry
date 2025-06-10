use crate::state::{Metadata, NameRecord};
use anchor_lang::prelude::*;

pub fn update_metadata(ctx: Context<UpdateMetadata>, metadata: Option<Metadata>) -> Result<()> {
    let record = &mut ctx.accounts.record;
    record.metadata = metadata;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(
        mut,
        seeds = [b"record", user.key().as_ref()],
        bump
    )]
    pub record: Account<'info, NameRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
