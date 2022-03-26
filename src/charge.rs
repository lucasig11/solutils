//! Functions, traits and macros for charging fees before instructions (currently only SPL
//! tokens).
//!
//! Use the [token_fee] function to charge a token before any instruction.
//! The account struct that describes the instruction must implement the [Chargeable] trait, which
//! can be derived using the [Chargeable](chargeable_derive::Chargeable) macro.
//!
//! The macro currently enforces that you either use specific name for the fields, or decorate them
//! with the corresponding attribute, but the errors should give you a hint on what should be done.
//!
//! ### Example:
//!
//! ```
//! use anchor_lang::prelude::*;
//! use common::charge::*;
//!
//! #[derive(Accounts, Chargeable)]
//! pub struct MyInstruction<'info> {
//!    #[account(mut)]
//!    #[fee_payer]
//!    // If this field was called `fee_payer`, then the attribute would not be required.
//!    pub owner: Signer<'info>,
//!    // Here the attribute can be omitted.
//!    #[account(mut)]
//!    pub fee_payer_ata: Account<'info, TokenAccount>,
//!    #[account(mut)]
//!    #[fee_incinerator_ata]
//!    pub incinerator: Account<'info, TokenAccount>,
//!    #[fee_token_address]
//!    pub token: Account<'info, Mint>,
//!    pub token_program: Program<'info, Token>,
//! }
//! ```
//!
//! And at the instruction handler:
//!
//! ```
//! use anchor_lang::prelude::*;
//! use common::charge::*;
//!
//! #[program]
//! pub mod my_program {
//!
//!     // Charges 100 tokens before issuing the instruction.
//!     #[access_control(token_fee(&ctx, 100))]
//!     pub fn my_instruction(ctx: Context<MyInstruction>) {
//!        /* ... */
//!     }
//! }
//! ```
use anchor_lang::{prelude::*, solana_program::incinerator};
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{Mint, Token, TokenAccount},
};
pub use chargeable_derive::Chargeable;

/// Trait for instruction accounts that can be charged with SPL tokens.
pub trait Chargeable<'info> {
    /// Mint address for the token that will be charged.
    fn mint_account(&self) -> &Account<Mint>;
    /// User associated token address.
    fn user_ata(&self) -> &Account<TokenAccount>;
    /// Incinerator associated token address.
    fn incinerator(&self) -> &Account<TokenAccount>;
    /// User's ATA authority.
    fn authority(&self) -> &Signer;
    /// SPL token program.
    fn token_program(&self) -> &Program<Token>;
}

/// Use a context's fields to charge a token fee before the instruction code execute. The amount
/// can be an argument from the instruction or a constant.
pub fn token_fee<'a, T: Chargeable<'a>>(ctx: &Context<T>, amount: u64) -> Result<()> {
    let user_ata = ctx.accounts.user_ata();
    let authority = ctx.accounts.authority();
    let mint_account = ctx.accounts.mint_account();
    let incinerator_ata = ctx.accounts.incinerator();

    verify_ata(incinerator_ata.key(), mint_account.key(), incinerator::id())?;
    verify_ata(user_ata.key(), mint_account.key(), authority.key())?;

    let accounts = anchor_spl::token::Transfer {
        from: user_ata.to_account_info(),
        to: incinerator_ata.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program().to_account_info(), accounts);

    anchor_spl::token::transfer(cpi_context, amount)?;

    Ok(())
}

fn verify_ata(ata: Pubkey, mint: Pubkey, user: Pubkey) -> Result<()> {
    require!(
        ata == get_associated_token_address(&user, &mint),
        FeeError::InvalidAssociatedTokenAddress
    );
    Ok(())
}

#[error_code]
pub enum FeeError {
    #[msg("Invalid associated token address.")]
    InvalidAssociatedTokenAddress = 1000,
}
