use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Not the owner")]
    OwnerError,
    #[msg("Signature verification failed.")]
    SigVerificationFailed,
    #[msg("Signature verification failed: 100")]
    SigVerificationFailed100,
    #[msg("Signature timeout.")]
    SigVerificationTimeout,
    #[msg("token_mint error.")]
    BrunError,
    #[msg("user_token_account error.")]
    AddJackpotError,
    #[msg("user_token_account error.")] 
    UserTokenAccount,
    #[msg("mint error.")] 
    Mint,
    #[msg("amount error.")] 
    Amount

}