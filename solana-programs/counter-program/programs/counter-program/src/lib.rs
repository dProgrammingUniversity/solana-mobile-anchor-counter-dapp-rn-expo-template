// Import necessary items from the Anchor framework and standard library
use anchor_lang::prelude::*;
use std::ops::DerefMut;

// Declare the program ID - this is your program's unique identifier on Solana
declare_id!("FfCxv78MgdXf9TvFzFVwXVuuYCqWUdFgAMdAnY97q5A8");

// Define the program module - this is where all the program's instructions live
#[program]
pub mod basic_counter {
    use super::*;

    // Initialize instruction: Creates a new counter account with initial count of 0
    // This is called when the program is first deployed
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = ctx.accounts.counter.deref_mut();
        let bump = ctx.bumps.counter;

        // Set initial state of the counter account
        *counter = Counter { count: 0, bump };

        Ok(())
    }

    // Increment instruction: Increases the counter by a specified amount
    // Amount must be between 1 and 100
    pub fn increment(ctx: Context<Increment>, amount: u64) -> Result<()> {
        // Validate that amount is within acceptable range
        require!(amount >= 1 && amount <= 100, ErrorCode::InvalidAmount);

        // Increase the counter by the specified amount
        ctx.accounts.counter.count += amount;
        Ok(())
    }
}

// Define the accounts needed for the Initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,            // Creates a new account
        payer = payer,   // The account that will pay for creation
        space = Counter::SIZE,  // Amount of space to allocate
        seeds = [b"counter"],   // PDA seed for deterministic address
        bump             // Bump seed for PDA
    )]
    counter: Account<'info, Counter>,  // The counter account to be created
    #[account(mut)]
    payer: Signer<'info>,             // The account paying for the transaction
    system_program: Program<'info, System>,  // Required for creating new accounts
}

// Define the accounts needed for the Increment instruction
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,                    // Account will be modified
        seeds = [b"counter"],   // PDA seed must match initialization
        bump = counter.bump     // Use stored bump from initialization
    )]
    counter: Account<'info, Counter>,  // The counter account to modify
}

// Define the structure of our Counter account
#[account]
pub struct Counter {
    pub count: u64,  // The current count value (8 bytes)
    pub bump: u8,    // The bump value for PDA (1 byte)
}

// Define the size needed for the Counter account
impl Counter {
    pub const SIZE: usize = 8 + 8 + 1;  // 8 (discriminator) + 8 (count) + 1 (bump)
}

// Define possible error codes for the program
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot get the bump.")]
    CannotGetBump,
    #[msg("The amount must be between 1 and 100.")]
    InvalidAmount,
}