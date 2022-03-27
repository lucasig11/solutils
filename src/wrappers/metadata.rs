//! Thin Anchor wrapper over [mpl_token_metadata] program.
//!
//! ## Example:
//! ```
//! # #[macro_use] extern crate anchor_lang;
//! use anchor_lang::prelude::*;
//! use solutils::wrappers::metadata::*;
//! use anchor_spl::token::TokenAccount;
//!
//! # fn main() {
//! #[derive(Accounts)]
//! pub struct AccountWithMetadata<'info> {
//!   pub token: Account<'info, TokenAccount>,
//!   #[account(
//!     seeds = ["metadata".as_ref(), token_metadata_program.key().as_ref(), token.key().as_ref()],
//!     seeds::program = token_metadata_program.key(),
//!     bump,
//!   )]
//!   pub metadata: Account<'info, MetadataAccount>,
//!   pub token_metadata_program: Program<'info, TokenMetadata>,
//! }
//! # }
//! ```
use anchor_lang::{
    prelude::*,
    solana_program::{self, entrypoint::ProgramResult},
};
use mpl_token_metadata::{
    instruction,
    state::{self, DataV2},
};

#[derive(Accounts)]
pub struct UpdateMetadataAccountV2<'info> {
    pub metadata_account: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
}

pub fn update_metadata_accounts_v2<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, UpdateMetadataAccountV2<'info>>,
    data: DataV2,
) -> ProgramResult {
    let ix = instruction::update_metadata_accounts_v2(
        mpl_token_metadata::ID,
        ctx.accounts.metadata_account.key(),
        ctx.accounts.update_authority.key(),
        None,
        Some(data),
        None,
        None,
    );

    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.metadata_account, ctx.accounts.update_authority],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Clone)]
/// Token metadata program struct.
pub struct TokenMetadata;

impl Id for TokenMetadata {
    fn id() -> Pubkey {
        mpl_token_metadata::ID
    }
}

#[derive(Clone)]
/// Wrapper for [mpl_token_metadata::state::Metadata] account.
pub struct MetadataAccount(state::Metadata);

impl MetadataAccount {
    pub const LEN: usize = state::MAX_METADATA_LEN;
}

impl AccountDeserialize for MetadataAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        state::Metadata::deserialize(buf)
            .map_err(|_| ErrorCode::AccountDidNotDeserialize.into())
            .map(MetadataAccount)
    }
}

impl AccountSerialize for MetadataAccount {}

impl Owner for MetadataAccount {
    fn owner() -> Pubkey {
        TokenMetadata::id()
    }
}

impl std::ops::Deref for MetadataAccount {
    type Target = state::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
