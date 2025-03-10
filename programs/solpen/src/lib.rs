use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;


declare_id!("BF8gXkDySmwV5LRsbLt9L8PDuypgVEX4s3cRJoeeKBpm");

#[program]
pub mod solpen {
    use super::*;

    pub fn binding(ctx: Context<Binding>, device_no:String) -> Result<()> {
        binding::handler(ctx, device_no)
    }

    pub fn redeem(ctx: Context<Redeem>, device_no:String, points:u64) -> Result<()> {
        redeem::handler(ctx, device_no, points)
    }

    pub fn ed25519_withdraw(ctx: Context<Verify>, device_no:String, number:u64, index:u32, end_time:u32, sig: [u8; 64], msg_string: String) -> Result<()> {
        ed25519_withdraw::handler(ctx, device_no, number, index, end_time, sig, msg_string)
    }

    pub fn init_pool(ctx: Context<InitPool>) -> Result<()> {
        init_pool::handler(ctx)
    }

    pub fn add_jackpot(ctx: Context<AddJackpot>, amount:u64) -> Result<()> {
        add_jackpot::handler(ctx, amount)
    }

    pub fn ai_init(ctx: Context<AiInit>, param: AiParam) -> Result<()> {
        ai_init::handler(ctx, param)
    }

    pub fn ai_fees(ctx: Context<AiFees>, amount: u64, msg_string: String) -> Result<()> {
        ai_fees::handler(ctx, amount, msg_string)
    }


    
}


