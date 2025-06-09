use crate::constants::*;
use crate::errors::NameRegistryError;
use crate::state::{Config, Metadata, NameRecord};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

pub fn register_name(
    ctx: Context<RegisterName>,
    name: String,
    metadata: Option<Metadata>,
) -> Result<()> {
    let record = &mut ctx.accounts.record;
    let user = &ctx.accounts.user;

    require!(name.len() <= MAX_NAME_LEN, NameRegistryError::NameTooLong);

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: user.to_account_info(),
                to: ctx.accounts.admin.to_account_info(),
            },
        ),
        REGISTRATION_LAMPORT_FEE,
    )?;

    record.owner = user.key();
    record.name = name;
    record.metadata = metadata;

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterName<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + NameRecord::MAX_SIZE,
        seeds = [b"record", user.key().as_ref()],
        bump
    )]
    pub record: Account<'info, NameRecord>,

    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
    /// CHECK: we read the address from `config.admin`
    #[account(mut, address = config.admin)]
    pub admin: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
