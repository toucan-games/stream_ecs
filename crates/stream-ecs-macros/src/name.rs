use proc_macro2::{Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Error, Ident};

pub fn crate_name_token(orig_name: &str) -> Result<TokenStream, Error> {
    let crate_name = crate_name(orig_name).map_err(|err| Error::new(Span::call_site(), err))?;
    let token = match crate_name {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident }
        }
    };
    Ok(token)
}
