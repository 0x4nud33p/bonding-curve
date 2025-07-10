use anchor_lang::prelude::*;

declare_id!("CUhDYmTTBNJeP7vCHGc8uTMc6gn9tbUcQHSyw49GXCDr");

#[program]
pub mod dtwitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
