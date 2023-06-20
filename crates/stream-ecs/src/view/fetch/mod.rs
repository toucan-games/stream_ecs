//! Utilities for fetches for the queries of ECS.

use crate::entity::Entity;

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
pub trait Fetch {
    /// Type of data which should be fetched.
    type Item<'a>
    where
        Self: 'a;

    // TODO add methods for the trait

    /// Fetches the data of the entity from the container.
    fn fetch(&mut self, entity: Entity) -> Option<Self::Item<'_>>;
}
