use {
    anchor_lang::prelude::*,
    crate::state::*,
    crate::errors::Errors,
    solana_program::pubkey::Pubkey,
    anchor_spl::token::{TransferChecked, transfer_checked},
    anchor_spl::token_interface,
    anchor_spl::token::ID as TOKEN_ID,
};


pub fn handler(ctx: Context<AiFees>, amount:u64, msg_string: String) -> Result<()> {
    require!(amount == ctx.accounts.ai_info.first_price || amount == ctx.accounts.ai_info.another_price, Errors::Amount);
    transfer_checked(ctx.accounts.transfer_checked_ctx(), amount, 6)?;
    msg!("ai_fees");
    msg!(&msg_string);
    msg!(&ctx.accounts.user.key().to_string());
    msg!(&amount.to_string());
    Ok(())
}

#[derive(Accounts)]
pub struct AiFees<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [AI_SEED.as_bytes()],
        bump
    )]
    pub ai_info: Account<'info, Ai>,

    #[account(
        mut,
        constraint = user_token_account.mint == ai_info.mint
        @ Errors::UserTokenAccount,
        token::token_program = TOKEN_ID
    )]
    pub user_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,

    #[account(
        mut,
        mint::token_program = TOKEN_ID,
        constraint = mint.key() == ai_info.mint
        @ Errors::Mint,
    )]
    pub mint: InterfaceAccount<'info, token_interface::Mint>,

    /// CHECK: This is the recipient account for SOL transfers
    #[account(mut, address = ai_info.recipient)] // Ensure recipient matches the Pubkey in pool_info
    pub recipient_account: AccountInfo<'info>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
} 

impl<'info> AiFees <'info> {
    
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.user_token_account.to_account_info(),
            to: self.recipient_account.to_account_info(),
            authority: self.user.to_account_info(),
            mint: self.mint.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

}