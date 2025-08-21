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

    pub fn set_params(
        ctx: Context<SetParams>,
        fee_receiver: Option<Pubkey>,
        initial_virtual_token_reserves: Option<u64>,
        initial_virtual_sol_reserves: Option<u64>,
        initial_real_token_reserves: Option<u64>,
        total_token_supply: Option<u64>,
        fee_basis_points: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.set_parameters(
            fee_receiver,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            total_token_supply,
            fee_basis_points,
        )
    }
}