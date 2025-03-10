use {
    anchor_lang::prelude::*,
    solana_program::pubkey::Pubkey,
};

pub const DEVICE_SEED: &str = "device";
pub const AI_SEED: &str = "ai";
pub const POOL_SEED: &str = "pool";
pub const VAULT_AUTH_SEED: &str = "vault_authority";
pub const REWARD_VAULT: &str = "reward_vault";



#[account]
pub struct Pool{
    pub owner: Pubkey,
    pub balance:u64,
    pub device_total:u32,
    pub points_total:u64,
    pub brun_mint:Pubkey,

    pub reward_mint:Pubkey,
    pub reward_total:u64,
    pub ed25519_withdraw:u64,
    pub signer25519: Pubkey
    
    
}


#[account]
pub struct Device {
    pub device_no: String,
    pub owner: Pubkey,
    pub points: u64,
    pub index:u32,
    pub ed25519_withdraw:u64
}

#[account]
pub struct Ai{
    pub recipient: Pubkey,
    pub first_price: u64,
    pub another_price: u64,
    pub mint: Pubkey,
}