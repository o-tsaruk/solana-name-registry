use crate::constants::*;
use crate::errors::NameRegistryError;
use crate::state::{Config, Metadata, Record, UserRecord};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

pub fn register_name(
    ctx: Context<RegisterName>,
    name: String,
    metadata: Option<Metadata>,
) -> Result<()> {
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

    let record = &mut ctx.accounts.record;
    record.owner = user.key();
    record.name = name.clone();
    record.metadata = metadata;

    let user_record = &mut ctx.accounts.user_record;
    user_record.name = name;

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct RegisterName<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Record::MAX_SIZE,
        seeds = [b"record", name.as_bytes()],
        bump
    )]
    pub record: Account<'info, Record>,

    #[account(
        init,
        payer = user,
        space = 8 + 64,
        seeds = [b"user_record", user.key().as_ref()],
        bump
    )]
    pub user_record: Account<'info, UserRecord>,

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
