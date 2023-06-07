//! Macros for yet another ECS implementation.

// TODO proper crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::Error;

mod component;
mod name;
mod resource;

/// Derive macro for `stream_ecs::Component` trait.
#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: TokenStream) -> TokenStream {
    component::derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive macro for `stream_ecs::Resource` trait.
#[proc_macro_derive(Resource)]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    resource::derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
