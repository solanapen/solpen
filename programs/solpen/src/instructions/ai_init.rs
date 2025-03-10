use {
    anchor_lang::prelude::*,
    crate::state::*,
    crate::errors::Errors,
    std::mem::size_of,
    solana_program::pubkey::Pubkey,
};


pub fn handler(ctx: Context<AiInit>, param: AiParam) -> Result<()> {
    let ai_info = &mut ctx.accounts.ai_info;
    ai_info.first_price = param.first_price;
    ai_info.another_price = param.another_price;
    ai_info.recipient = param.recipient;
    ai_info.mint = param.mint;
    Ok(())
}

#[derive(Accounts)]
pub struct AiInit<'info>{

    #[account(
        seeds = [POOL_SEED.as_bytes()],
        bump,
        constraint = pool_info.owner == user.key() @Errors::OwnerError
    )]
    pub pool_info: Account<'info, Pool>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [AI_SEED.as_bytes()],
        bump,
        payer = user,
        space = 8 + size_of::<Ai>()
    )]
    pub ai_info: Account<'info, Ai>,
    pub system_program: Program<'info, System>,
} 

#[account]
pub struct AiParam{
    pub recipient: Pubkey,
    pub first_price: u64,
    pub another_price: u64,
    pub mint: Pubkey,
}