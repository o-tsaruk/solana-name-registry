use crate::state::{Metadata, Record};
use anchor_lang::prelude::*;

pub fn update_metadata(
    ctx: Context<UpdateMetadata>,
    _name: String,
    metadata: Option<Metadata>,
) -> Result<()> {
    let record = &mut ctx.accounts.record;
    record.metadata = metadata;
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateMetadata<'info> {
    #[account(
        mut,
        seeds = [b"record", name.as_bytes()],
        bump
    )]
    pub record: Account<'info, Record>,
    #[account(mut)]
    pub user: Signer<'info>,
}
