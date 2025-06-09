use anchor_lang::prelude::*;

pub fn transfer_name(_ctx: Context<TransferName>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct TransferName {}
