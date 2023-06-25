//! Number of useful procedural macros for `stream-ecs` crate.

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::Error;

mod component;
mod name;
mod resource;

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: TokenStream) -> TokenStream {
    component::derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Resource, attributes(resource))]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    resource::derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
