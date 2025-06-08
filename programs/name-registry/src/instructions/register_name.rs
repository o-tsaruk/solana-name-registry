use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

pub fn register_name(ctx: Context<RegisterName>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterName {}
