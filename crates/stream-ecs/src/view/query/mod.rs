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

    /// Fetches the data of the entity from the fetcher.
    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>>;
}

// TODO divide into mutable and immutable queries
