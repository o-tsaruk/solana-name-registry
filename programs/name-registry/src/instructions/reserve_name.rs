use anchor_lang::prelude::*;

pub fn reserve_name(ctx: Context<ReserveName>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ReserveName {}
