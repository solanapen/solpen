use {
    anchor_lang::prelude::*,
    crate::state::*,
    crate::errors::Errors,
    std::mem::size_of,
    anchor_spl::token_interface,
    anchor_spl::token::{Burn, burn}
};


pub fn handler(ctx: Context<Binding>, device_no:String) -> Result<()> {
    msg!("binding");
    msg!(&device_no);
    let device_info = &mut ctx.accounts.device_info;
    device_info.owner = ctx.accounts.user.key();
    device_info.device_no = device_no;
    device_info.points = 0;
    let pool_info = &mut ctx.accounts.pool_info;
    pool_info.device_total += 1;
    Ok(())
}

#[derive(Accounts)]
#[instruction(device_no: String)]
pub struct Binding<'info>{

    
    #[account(
        seeds = [POOL_SEED.as_bytes()],
        bump,
        constraint = pool_info.brun_mint.as_ref() == token_mint.key().as_ref() @Errors::BrunError
    )]
    pub pool_info: Account<'info, Pool>,
    #[account(
        init,
        seeds = [DEVICE_SEED.as_bytes(), device_no.as_bytes()],
        bump,
        payer = user,
        space = 8 + size_of::<Device>()
    )]
    pub device_info: Account<'info, Device>,

    #[account(
        mut,
        token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
} 
