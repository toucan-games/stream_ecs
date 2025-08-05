use deluxe::{ExtractAttributes, extract_attributes};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Result};

use crate::name::crate_name_token;

#[derive(ExtractAttributes)]
#[deluxe(attributes(resource))]
struct ResourceAttributes {
    #[deluxe(default = None)]
    #[deluxe(rename = crate)]
    crate_name: Option<Ident>,
}

pub fn derive<Input>(input: Input) -> Result<TokenStream>
where
    Input: Into<TokenStream>,
{
    let input = input.into();
    let mut input = syn::parse2(input)?;

    let ResourceAttributes { crate_name } = extract_attributes(&mut input)?;
    let crate_name = match crate_name {
        Some(crate_name) => quote! { #crate_name },
        None => crate_name_token("stream-ecs")?,
    };
    let trait_ident = quote! { #crate_name::resource::Resource };

    let DeriveInput {
        ident, generics, ..
    } = input;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {}
    };
    Ok(output)
}
