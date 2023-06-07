use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Result};

use crate::name::crate_name_token;

#[derive(FromDeriveInput)]
#[darling(attributes(component), forward_attrs(allow, doc, cfg))]
struct ComponentOptions {
    storage: Ident,
}

pub fn derive<Input>(input: Input) -> Result<TokenStream>
where
    Input: Into<TokenStream>,
{
    let input = syn::parse2(input.into())?;

    let ComponentOptions { storage } = FromDeriveInput::from_derive_input(&input)?;
    let DeriveInput {
        ident, generics, ..
    } = input;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let crate_name = crate_name_token("stream-ecs");
    let trait_ident = quote! { #crate_name::component::Component };

    let output = quote! {
        impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
            type Storage = #storage;
        }
    };
    Ok(output)
}
