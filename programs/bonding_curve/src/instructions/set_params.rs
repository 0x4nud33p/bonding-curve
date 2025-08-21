use anchor::prelude::*;

use create::errors::Errors;
use crate::state::Global;
use crate::events::ParamsSet;
use crate::GLOBAL;

#[derive(Accounts)]
pub struct SetParams<'info> {
    #[account(
        mut,
        seeds = [GLOBAL],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,
    #[account(
        mut,
        address = global.authority,
    )]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetParams<'info> {
    pub fn set_parameters(
        &mut self,
        fee_receiver: Option<Pubkey>,
        initial_virtual_token_reserves: Option<u64>,
        initial_virtual_sol_reserves: Option<u64>,
        initial_real_token_reserves: Option<u64>,
        total_token_supply: Option<u64>,
        fee_basis_points: Option<u64>,
    ) -> Result<()> {
        require!(fee_basis_points.unwrap_or(self.global.fee_basis_points) <= 500, Errors::InvalidCalculation);

        self.global.fee_receiver = fee_receiver.unwrap_or(self.global.fee_receiver);
        self.global.initial_virtual_token_reserves = initial_virtual_token_reserves.unwrap_or(self.global.initial_virtual_token_reserves);
        self.global.initial_virtual_sol_reserves = initial_virtual_sol_reserves.unwrap_or(self.global.initial_virtual_sol_reserves);
        self.global.initial_real_token_reserves = initial_real_token_reserves.unwrap_or(self.global.initial_real_token_reserves);
        self.global.total_token_supply = total_token_supply.unwrap_or(self.global.total_token_supply);
        self.global.fee_basis_points = fee_basis_points.unwrap_or(self.global.fee_basis_points);
        
        emit!(ParamsSet {
            fee_receiver: self.global.fee_receiver,
            initial_virtual_token_reserves: self.global.initial_virtual_token_reserves,
            initial_virtual_sol_reserves: self.global.initial_virtual_sol_reserves,
            initial_real_token_reserves: self.global.initial_real_token_reserves,
            total_token_supply: self.global.total_token_supply,
            fee_basis_points: self.global.fee_basis_points,
        });
        Ok(())
    }
}