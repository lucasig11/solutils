//! Thin Anchor wrapper over [mpl_token_metadata] program.
//!
//! ## Example:
//! ```
//! # #[macro_use] extern crate anchor_lang; use anchor_lang::prelude::*;
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
use anchor_lang::{prelude::*, solana_program};
use mpl_token_metadata::state::{self, DataV2};

#[derive(Accounts)]
pub struct UpdateMetadataAccountV2<'info> {
    pub metadata_account: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FreezeDelegatedAccount<'info> {
    pub delegate: AccountInfo<'info>,
    pub token_account: AccountInfo<'info>,
    pub edition: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ThawDelegatedAccount<'info> {
    pub delegate: AccountInfo<'info>,
    pub token_account: AccountInfo<'info>,
    pub edition: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
}

pub fn update_metadata_accounts_v2<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, UpdateMetadataAccountV2<'info>>,
    data: DataV2,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::update_metadata_accounts_v2(
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

pub fn freeze_delegated_account<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, FreezeDelegatedAccount<'info>>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::freeze_delegated_account(
        mpl_token_metadata::id(),
        ctx.accounts.delegate.key(),
        ctx.accounts.token_account.key(),
        ctx.accounts.edition.key(),
        ctx.accounts.mint.key(),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.delegate.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn thaw_delegated_account<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, ThawDelegatedAccount<'info>>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::thaw_delegated_account(
        mpl_token_metadata::id(),
        ctx.accounts.delegate.key(),
        ctx.accounts.token_account.key(),
        ctx.accounts.edition.key(),
        ctx.accounts.mint.key(),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.delegate.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
        ],
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

#[derive(Clone)]
/// Wrapper for [mpl_token_metadata::state::Edition] account.
pub struct EditionAccount(state::Edition);

impl EditionAccount {
    pub const LEN: usize = state::MAX_EDITION_LEN;
}

impl AccountDeserialize for EditionAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        state::Edition::deserialize(buf)
            .map_err(|_| ErrorCode::AccountDidNotDeserialize.into())
            .map(Self)
    }
}

impl AccountSerialize for EditionAccount {}

impl Owner for EditionAccount {
    fn owner() -> Pubkey {
        TokenMetadata::id()
    }
}

impl std::ops::Deref for EditionAccount {
    type Target = state::Edition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
/// Wrapper for [mpl_token_metadata::state::MasterEditionV2] account.
pub struct MasterEditionAccount(state::MasterEditionV2);

impl MasterEditionAccount {
    pub const LEN: usize = state::MAX_MASTER_EDITION_LEN;
}

impl AccountDeserialize for MasterEditionAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        state::MasterEditionV2::deserialize(buf)
            .map_err(|_| ErrorCode::AccountDidNotDeserialize.into())
            .map(Self)
    }
}

impl AccountSerialize for MasterEditionAccount {}

impl Owner for MasterEditionAccount {
    fn owner() -> Pubkey {
        TokenMetadata::id()
    }
}

impl std::ops::Deref for MasterEditionAccount {
    type Target = state::MasterEditionV2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
