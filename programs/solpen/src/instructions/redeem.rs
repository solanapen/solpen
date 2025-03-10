use {
    anchor_lang::prelude::*,
    crate::{state::*, errors::*},
    solana_program::pubkey::Pubkey,
};

pub fn handler(ctx: Context<Redeem>, _device_no:String, points:u64) -> Result<()> {

    let device_info = &mut ctx.accounts.device_info;
    device_info.points += points;
    Ok(())
}


#[derive(Accounts)]
#[instruction(_device_no: String)]
pub struct Redeem<'info>{

    #[account(
        mut,
        seeds = [DEVICE_SEED.as_bytes(), _device_no.as_bytes()],
        bump,
        constraint = device_info.owner == user.key() @Errors::OwnerError
    )]
    pub device_info: Account<'info, Device>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
} 

