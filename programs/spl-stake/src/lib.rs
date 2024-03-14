use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spl_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin:Signer<'info>,
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