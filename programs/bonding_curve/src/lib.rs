use anchor_lang::prelude::*;

declare_id!("CUhDYmTTBNJeP7vCHGc8uTMc6gn9tbUcQHSyw49GXCDr");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        ctx.accounts.initialize_global_config(
            &ctx.bumps,
        )
    }
}