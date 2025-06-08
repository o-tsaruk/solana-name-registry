use anchor_lang::prelude::*;

pub fn set_metadata(ctx: Context<SetMetadata>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct SetMetadata {}
