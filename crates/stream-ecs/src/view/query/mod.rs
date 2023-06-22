//! Utilities for queries of ECS.

use crate::{component::registry::Registry as Components, entity::Entity};

mod impls;

/// Type of query to be queried from components by view.
pub trait Query {
    /// Type of result yielded by the query.
    type Item<'item>;

    /// Type that fetches query item from itself.
    type Fetch<'fetch>;

    /// Creates new fetcher from provided component registry.
    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components;

    /// Fetches data of the entity from the fetcher.
    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>>;
}

/// Type of query which is readonly, or has no mutable access to data.
pub trait ReadonlyQuery: Query {
    /// Creates new fetcher from provided component registry.
    fn new_readonly_fetch<C>(components: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components;

    /// Fetches data of the entity from the fetcher.
    fn readonly_fetch<'fetch>(
        fetch: &Self::Fetch<'fetch>,
        entity: Entity,
    ) -> Option<Self::Item<'fetch>>;
}
