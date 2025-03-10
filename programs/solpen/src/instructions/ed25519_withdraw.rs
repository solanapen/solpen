use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use solana_program::ed25519_program::ID as ED25519_ID;
use crate::errors::Errors;
use crate::state::*;
use anchor_spl::token_interface;
use anchor_spl::token::ID as TOKEN_ID;
use spl_token::instruction::transfer_checked;
use solana_program::program::invoke_signed;
use solana_program::sysvar::instructions::{ID as IX_ID, load_instruction_at_checked};

use std::convert::TryInto;
use solana_program::pubkey::Pubkey;

 /// External instruction that only gets executed if
    /// an `Ed25519Program.createInstructionWithPublicKey`
    /// instruction was sent in the same transaction.
    pub fn handler(ctx: Context<Verify>, device_no:String, amount:u64, index:u32, end_time:u32, sig: [u8; 64], msg_string: String) -> Result<()> {
        if index <= ctx.accounts.device_info.index  || 
        i64::from(end_time) < Clock::get().unwrap().unix_timestamp
        {
            return Err(Errors::SigVerificationTimeout.into()); 
        }
           
        // Get what should be the Ed25519Program instruction
        let ix: Instruction = load_instruction_at_checked(2, &ctx.accounts.sysvar)?;
        //
        
        // let mut msg = Pubkey::new_from_array(pubkey).to_string();
        let mut msg = ctx.accounts.user.key().to_string();
        let _ = &msg.push_str(&device_no);
        let _ = &msg.push_str(&amount.to_string());
        let _ = &msg.push_str(&index.to_string());
        let _ = &msg.push_str(&end_time.to_string());
        msg!(&msg_string);
        msg!(&amount.to_string());
        // Check that ix is what we expect to have been sent
        verify_ed25519_ix(&ix, &ctx.accounts.pool_info.signer25519.key().to_bytes(), &msg.as_bytes(), &sig)?;

        let device_info = &mut ctx.accounts.device_info;
        device_info.index = index;
        send_reward(ctx, amount)?;
        Ok(())
    }


/// Verify Ed25519Program instruction fields
pub fn verify_ed25519_ix(ix: &Instruction, pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
    if  ix.program_id       != ED25519_ID                   ||  // The program id we expect
        ix.accounts.len()   != 0                            ||  // With no context accounts
        ix.data.len()       != (16 + 64 + 32 + msg.len())       // And data of this size
    {
        return Err(Errors::SigVerificationFailed.into());    // Otherwise, we can already throw err
    }

    check_ed25519_data(&ix.data, pubkey, msg, sig)?;            // If that's not the case, check data

    Ok(())
}

/// Verify serialized Ed25519Program instruction data
pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
    // According to this layout used by the Ed25519Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33

    // "Deserializing" byte slices

    let num_signatures                  = &[data[0]];        // Byte  0
    let padding                         = &[data[1]];        // Byte  1
    let signature_offset                = &data[2..=3];      // Bytes 2,3
    let signature_instruction_index     = &data[4..=5];      // Bytes 4,5
    let public_key_offset: &[u8]               = &data[6..=7];      // Bytes 6,7
    let public_key_instruction_index    = &data[8..=9];      // Bytes 8,9
    let message_data_offset             = &data[10..=11];    // Bytes 10,11
    let message_data_size               = &data[12..=13];    // Bytes 12,13
    let message_instruction_index       = &data[14..=15];    // Bytes 14,15

    let data_pubkey                     = &data[16..16+32];  // Bytes 16..16+32
    let data_sig                        = &data[48..48+64];  // Bytes 48..48+64
    let data_msg                        = &data[112..];      // Bytes 112..end

    // Expected values

    let exp_public_key_offset:      u16 = 16; // 2*u8 + 7*u16
    let exp_signature_offset:       u16 = exp_public_key_offset + pubkey.len() as u16;
    let exp_message_data_offset:    u16 = exp_signature_offset + sig.len() as u16;
    let exp_num_signatures:          u8 = 1;
    let exp_message_data_size:      u16 = msg.len().try_into().unwrap();

    // Header and Arg Checks

    // Header
    if  num_signatures                  != &exp_num_signatures.to_le_bytes()        ||
        padding                         != &[0]                                     ||
        signature_offset                != &exp_signature_offset.to_le_bytes()      ||
        signature_instruction_index     != &u16::MAX.to_le_bytes()                  ||
        public_key_offset               != &exp_public_key_offset.to_le_bytes()     ||
        public_key_instruction_index    != &u16::MAX.to_le_bytes()                  ||
        message_data_offset             != &exp_message_data_offset.to_le_bytes()   ||
        message_data_size               != &exp_message_data_size.to_le_bytes()     ||
        message_instruction_index       != &u16::MAX.to_le_bytes()  
    {
        return Err(Errors::SigVerificationFailed.into());
    }

    // Arguments
    if  data_pubkey != pubkey   ||
        data_msg    != msg      ||
        data_sig    != sig
    {
        return Err(Errors::SigVerificationFailed.into());
    }

    Ok(())
}


pub fn send_reward(ctx: Context<Verify>, amount:u64) -> Result<()> {
    let auth_bump = ctx.bumps.pool_authority;//ctx.accounts.pool_info.reward_auth_bump;
    let auth_seeds = &[VAULT_AUTH_SEED.as_bytes(), &[auth_bump]];
    let signer = &[&auth_seeds[..]];
    // transfer_checked(ctx.accounts.transfer_checked_ctx(), amount, 1)?;
    // let pool_info = &mut ctx.accounts.pool_info;
    // transfer out_amount from stake vault to user
    let transfer_ix: solana_program::instruction::Instruction = transfer_checked(
        &ctx.accounts.token_program.key(),
        &ctx.accounts.reward_vault.key(),
        &ctx.accounts.reward_mint.key(),
        &ctx.accounts.user_token_account.key(),
        &ctx.accounts.pool_authority.key(),
        &[&ctx.accounts.pool_authority.key()],
        amount,
        6
    ).unwrap();
    invoke_signed(
        &transfer_ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.reward_vault.to_account_info(),
            ctx.accounts.reward_mint.to_account_info(),
            ctx.accounts.user_token_account.to_account_info(),
            ctx.accounts.pool_authority.to_account_info(),
        ],
        signer
    )?;
    let pool_info = &mut ctx.accounts.pool_info;
    pool_info.ed25519_withdraw += &amount;
    let device_info =  &mut ctx.accounts.device_info;
    device_info.ed25519_withdraw += amount;

    Ok(())
}



#[derive(Accounts)]
#[instruction(device_no: String)]
pub struct Verify<'info> {

    #[account(
        seeds = [POOL_SEED.as_bytes()],
        bump
    )]
    pub pool_info: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [DEVICE_SEED.as_bytes(), device_no.as_bytes()],
        bump,
        constraint = device_info.owner == user.key() @Errors::OwnerError
    )]
    pub device_info: Account<'info, Device>,

    /// CHECK
    #[account(
        seeds = [VAULT_AUTH_SEED.as_bytes()],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    pub user: Signer<'info>,
    #[account(
        mut,
        mint::token_program = TOKEN_ID
    )]
    pub reward_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(
        mut,
        constraint = user_token_account.mint == pool_info.reward_mint
        @ Errors::AddJackpotError,
        token::token_program = TOKEN_ID
    )]
    pub user_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        seeds = [reward_mint.key().as_ref(), pool_authority.key().as_ref()],
        bump
    )]
    pub reward_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,

    /// CHECK: The address check is needed because otherwise
    /// the supplied Sysvar could be anything else.
    /// The Instruction Sysvar has not been implemented
    /// in the Anchor framework yet, so this is the safe approach.
    #[account(address = IX_ID)]
    pub sysvar: AccountInfo<'info>,
}

