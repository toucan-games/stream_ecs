//! Utilities for queries of ECS.

use crate::entity::Entity;

mod impls;

/// Type of query to be queried from components by view.
pub trait Query {
    /// Type of result yielded by the query.
    type Item<'a>;

    /// Type that fetches query item from itself.
    type Fetch<'a>;

    /// Fetches the data of the entity from the fetcher.
    fn fetch<'a>(fetch: &'a mut Self::Fetch<'_>, entity: Entity) -> Option<Self::Item<'a>>;
}

// TODO divide into mutable and immutable queries
