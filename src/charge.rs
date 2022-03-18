use anchor_lang::{prelude::*, solana_program::incinerator};
pub use anchor_spl::{
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

#[inline]
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
