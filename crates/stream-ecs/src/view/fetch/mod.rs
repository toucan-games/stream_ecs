//! Utilities for fetches for the queries of ECS.

pub use self::{
    component::FetchComponent, component_mut::FetchComponentMut, entity::FetchEntity,
    option::FetchOption,
};

mod component;
mod component_mut;
mod entity;
mod impls;
mod option;

/// Fetcher of the data retrieved from entity or component registries.
pub trait Fetch<'a>: 'a {
    /// Type of data which should be fetched.
    type Item: 'a;

    // TODO add methods for the trait
}
