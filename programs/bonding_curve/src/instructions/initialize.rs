use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::Global;
use crate::events::Initialized;

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Global::INIT_SPACE,
        seeds = [GLOBAL],
        bump
    )]
    pub global: Account<'info, Global>,
    #[account(
        mut,
        address = ADMIN
    )]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


impl<'info> InitializeAccount<'info> {
    pub fn initialize_global_config(&mut self, bump: &InitializeBumps) -> Result<()> {
        self.global.set_inner(Global, {
            initialized: true,
            paused: false,
            authority: self.user.key(),
            fee_receiver: self.user.key(),
            initial_virtual_token_reserves: P * SCALE,
            initial_virtual_sol_reserves: R * LAMPORTS_PER_SOL,
            initial_real_token_reserves: BONDING_CURVE_SUPPLY,
            total_token_supply: TOTAL_SUPPLY,
            fee_basis_points: 100,
            bump: bump.global,
        });

        emit!(Initialized, {
            authority: self.user.key(),
            fee_receiver: self.user.key()
        });

        Ok()
    }
}