use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma};

/// Implements the `Chargeable` trait for an instruction account.
#[proc_macro_derive(
    Chargeable,
    attributes(
        fee_payer,
        fee_payer_ata,
        fee_token_address,
        fee_incinerator_ata,
        token_program
    )
)]
pub fn chargeable_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    impl_chargeable(input)
}

macro_rules! get_fields {
    ($(let $name:ident),*; $fields:expr; $span:expr) => {
        $(
            let $name = match get_field(stringify!($name), $fields) {
                Ok(field) => field.ident.as_ref().unwrap(),
                Err(err) => return syn::Error::new($span, err).to_compile_error().into(),
            };
        )*
    };
}

fn get_field<'a>(
    name: &str,
    fields: &'a Punctuated<syn::Field, Comma>,
) -> Result<&'a syn::Field, String> {
    fields
        .iter()
        .find(|field| {
            field.ident.as_ref().map(|ident| ident.to_string()) == Some(name.to_string())
                || field.attrs.iter().any(|attr| {
                    attr.path.get_ident().map(|ident| ident.to_string()) == Some(name.to_string())
                })
        })
        .ok_or(format!("#[derive(Chargeable)] Missing `{}` field. Add it to the struct or use the #[{0}] attribute in an existing field.", name))
}

fn impl_chargeable(item_struct: syn::ItemStruct) -> TokenStream {
    let name = &item_struct.ident;

    let fields = match item_struct.fields {
        syn::Fields::Named(syn::FieldsNamed { ref named, .. }) => named,
        _ => {
            return syn::Error::new(
                name.span(),
                "#[derive(Chargeable)] is only defined for structs with named fields.",
            )
            .to_compile_error()
            .into();
        }
    };

    get_fields!(
        let fee_payer,
        let fee_payer_ata,
        let fee_incinerator_ata,
        let fee_token_address,
        let token_program;
        fields;
        name.span()
    );

    quote!(
        impl<'info> crate::Chargeable<'info> for #name<'info> {
            fn mint_account(&self) -> &'info anchor_lang::accounts::account::Account<anchor_spl::token::Mint> {
                &self.#fee_token_address
            }

            fn user_ata(&self) -> &'info anchor_lang::accounts::account::Account<anchor_spl::token::TokenAccount> {
                &self.#fee_payer_ata
            }

            fn incinerator(&self) -> &'info anchor_lang::accounts::account::Account<anchor_spl::token::TokenAccount> {
                &self.#fee_incinerator_ata
            }

            fn authority(&self) -> &'info anchor_lang::accounts::signer::Signer {
                &self.#fee_payer
            }

            fn token_program(&self) -> &'info anchor_lang::accounts::program::Program<anchor_spl::token::Token> {
                &self.#token_program
            }
        }
    )
    .into()
}
