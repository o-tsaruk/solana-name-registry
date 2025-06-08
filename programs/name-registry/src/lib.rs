use anchor_lang::prelude::*;

declare_id!("8ut3Sq75sPmJweJFi9ZLWVEt6rhaneETWRromG9V8Laj");

#[program]
pub mod name_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized with id: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
