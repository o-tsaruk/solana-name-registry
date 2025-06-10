#![allow(unexpected_cfgs, deprecated)]

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::Metadata;

declare_id!("8ut3Sq75sPmJweJFi9ZLWVEt6rhaneETWRromG9V8Laj");

#[program]
pub mod name_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::initialize(ctx)
    }

    pub fn register_name(
        ctx: Context<RegisterName>,
        name: String,
        metadata: Option<Metadata>,
    ) -> Result<()> {
        register_name::register_name(ctx, name, metadata)
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>, metadata: Option<Metadata>) -> Result<()> {
        update_metadata::update_metadata(ctx, metadata)
    }

    pub fn transfer_name(ctx: Context<TransferName>) -> Result<()> {
        transfer_name::transfer_name(ctx)
    }
}
