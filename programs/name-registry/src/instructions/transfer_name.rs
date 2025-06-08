use anchor_lang::prelude::*;

pub fn transfer_name(ctx: Context<TransferName>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct TransferName {}
