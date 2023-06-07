use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::name::crate_name_token;

pub fn derive<Input>(input: Input) -> Result<TokenStream>
where
    Input: Into<TokenStream>,
{
    let input = syn::parse2(input.into())?;

    let DeriveInput {
        ident, generics, ..
    } = input;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let crate_name = crate_name_token("stream-ecs");
    let trait_ident = quote! { #crate_name::resource::Resource };

    let output = quote! {
        impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {}
    };
    Ok(output)
}
