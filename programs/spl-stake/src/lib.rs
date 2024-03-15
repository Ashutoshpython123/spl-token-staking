use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spl_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, start_slot: u64, end_slot: u64) -> Result<()> {
        msg!("initialized.");

        let pool_info = &mut ctx.accounts.pool_info;
        pool_info.admin = ctx.accounts.admin.key();
        pool_info.token = ctx.accounts.staking_token.key();
        pool_info.start_slot = start_slot;
        pool_info.end_slot = end_slot;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin:Signer<'info>,
    #[account(init, payer = admin, space = 8 + PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<info, TokenAccount>,
    pub system_program: Program<'info, System>
}


#[account]
pub struct PoolInfo {
    pub admin: Pubkey,
    pub token: Pubkey,
    pub start_slot: u64,
    pub end_slot: u64    
}

#[account]
pub struct UserInfo {
    pub amount : u64,
    pub reward_claimed : u64,
    pub deposit_slot : u64
}

impl UserInfo {
    pub const LEN: usize = 8 + 8 + 8;
}

impl PoolInfo  {
    pub const LEN: usize = 32 + 32 + 8 + 8;
}