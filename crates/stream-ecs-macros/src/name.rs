use proc_macro2::{Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::Ident;

pub fn crate_name_token(orig_name: &str) -> TokenStream {
    let Ok(crate_name) = crate_name(orig_name) else {
        panic!("`{orig_name}` should present in `Cargo.toml`")
    };
    match crate_name {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { #ident }
        }
    }
}
