//! Utilities for queries of ECS.

use super::fetch::Fetch;

mod impls;

/// Type of query to be queried from components by view.
pub trait Query {
    /// Type of result yielded by the query.
    type Item<'a>;

    /// Type that fetches query item from the container.
    type Fetch<'a>: Fetch<'a, Item = Self::Item<'a>>;
}