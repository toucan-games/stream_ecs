//! Macros for yet another ECS implementation.

// TODO proper crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;

mod component;
mod resource;

/// Derive macro for `stream_ecs::Component` trait.
#[proc_macro_derive(Component)]
pub fn component_derive(annotated_item: TokenStream) -> TokenStream {
    annotated_item
}

/// Derive macro for `stream_ecs::Resource` trait.
#[proc_macro_derive(Resource)]
pub fn resource_derive(annotated_item: TokenStream) -> TokenStream {
    annotated_item
}
