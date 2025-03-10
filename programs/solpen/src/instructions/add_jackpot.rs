use {
    anchor_lang::prelude::*,
    crate::state::*,
    solana_program::pubkey::Pubkey,
    anchor_spl::token_interface,
    anchor_spl::token::{TransferChecked, transfer_checked},
    anchor_spl::token::ID as TOKEN_ID,
    crate::errors::Errors
};


pub fn handler(ctx: Context<AddJackpot>, amount:u64) -> Result<()> {
    transfer_checked(ctx.accounts.transfer_checked_ctx(), amount, 6)?;
    msg!("Pool initial total: {}", ctx.accounts.pool_info.reward_total);
    msg!("User entry initial balance: {}", ctx.accounts.user_token_account.amount);
    let pool_info = &mut ctx.accounts.pool_info;
    pool_info.reward_total += amount;
    msg!("Current pool stake total: {}", pool_info.reward_total);
    Ok(())
}·


#[derive(Accounts)]
pub struct AddJackpot<'info>{

    #[account(
        seeds = [POOL_SEED.as_bytes()],
        bump
    )]
    pub pool_info: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,

    // PDA, auth over all token vaults
    /// CHECK: unsafe
    #[account(
        seeds = [VAULT_AUTH_SEED.as_bytes()],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [reward_mint.key().as_ref(), pool_authority.key().as_ref()],
        bump
    )]
    pub reward_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        constraint = user_token_account.mint == pool_info.reward_mint
        @ Errors::AddJackpotError,
        token::token_program = TOKEN_ID
    )]
    pub user_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        mint::token_program = TOKEN_ID
    )]
    pub reward_mint: InterfaceAccount<'info, token_interface::Mint>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
} 


impl<'info> AddJackpot <'info> {
    // transfer_checked for Token2022
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.user_token_account.to_account_info(),
            to: self.reward_vault.to_account_info(),
            authority: self.user.to_account_info(),
            mint: self.reward_mint.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
