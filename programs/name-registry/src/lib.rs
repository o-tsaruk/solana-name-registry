#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("8ut3Sq75sPmJweJFi9ZLWVEt6rhaneETWRromG9V8Laj");

#[program]
pub mod name_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized with id: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn register_name(ctx: Context<RegisterName>) -> Result<()> {
        register_name::register_name(ctx)
    }

    pub fn set_metadata(ctx: Context<SetMetadata>) -> Result<()> {
        set_metadata::set_metadata(ctx)
    }

    pub fn transfer_name(ctx: Context<TransferName>) -> Result<()> {
        transfer_name::transfer_name(ctx)
    }

    pub fn reserve_name(ctx: Context<ReserveName>) -> Result<()> {
        reserve_name::reserve_name(ctx)
    }
}

#[account]
pub struct Config {
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>, // payer and initial admin
    pub system_program: Program<'info, System>,
}
