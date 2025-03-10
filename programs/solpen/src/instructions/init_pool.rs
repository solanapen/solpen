use {
    anchor_lang::prelude::*,
    crate::state::*,
    std::mem::size_of,
    solana_program::pubkey::Pubkey,
    anchor_spl::token_interface,
    anchor_spl::token::ID as TOKEN_ID
};


pub fn handler(ctx: Context<InitPool>) -> Result<()> {
    let pool_info = &mut ctx.accounts.pool_info;
    pool_info.owner = ctx.accounts.user.key();
    pool_info.brun_mint = ctx.accounts.brun_mint.key();
    pool_info.reward_mint = ctx.accounts.reward_mint.key();
    pool_info.signer25519 = ctx.accounts.signer25519.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitPool<'info>{

    #[account(
        init,
        seeds = [POOL_SEED.as_bytes()],
        bump,
        payer = user,
        space = 8 + size_of::<Pool>()
    )]
    pub pool_info: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        mint::token_program = TOKEN_ID
    )]
    pub brun_mint: InterfaceAccount<'info, token_interface::Mint>,

    // PDA, auth over all token vaults
    /// CHECK: unsafe
    #[account(
        seeds = [VAULT_AUTH_SEED.as_bytes()],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    #[account(
        init,
        token::mint = reward_mint,
        token::authority = pool_authority,
        // use token_mint, pool auth, and constant as seeds for token a vault
        seeds = [reward_mint.key().as_ref(), pool_authority.key().as_ref()],
        bump,
        payer = user,
    )]
    pub reward_vault: InterfaceAccount<'info, token_interface::TokenAccount>,

    #[account(mut)]
    pub signer25519: Signer<'info>,

    #[account(
        mut,
        mint::token_program = TOKEN_ID
    )]
    pub reward_mint: InterfaceAccount<'info, token_interface::Mint>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
} 