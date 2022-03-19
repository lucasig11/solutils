//! Thin Anchor wrapper over [mpl_token_metadata] program.
//!
//! ## Example:
//! ```
//! # use anchor_lang::prelude::*;
//! use anchor_spl::TokenAccount;
//!
//! #[derive(Account)]
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
//! ```
use anchor_lang::prelude::*;
use mpl_token_metadata::state;

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
