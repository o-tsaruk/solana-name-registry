use crate::state::Config;
use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = *ctx.accounts.user.key;
    msg!("Initialized with id: {:?}", ctx.program_id);
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
      init,
      payer = user,
      space = 8 + 32,
      seeds = [b"config"],
      bump
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>, // initial admin
    pub system_program: Program<'info, System>,
}
