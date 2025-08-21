use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Global {
    pub initialized: bool,
    pub paused: bool,
    pub authority: Pubkey,
    pub fee_receiver: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub total_token_supply: u64,
    pub fee_basis_points: u64,
    pub bump: u8,
}