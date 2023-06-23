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
pub trait ReadonlyQuery: AsReadonly<Readonly = Self> {
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

/// Extension of query which allows to convert this query into readonly query.
pub trait IntoReadonly: Query {
    /// Readonly variant of this query.
    type Readonly: ReadonlyQuery;

    /// Converts the fetcher of this query into a readonly fetcher.
    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_>;
}

/// Extension of query which allows to borrow this query as readonly query.
pub trait AsReadonly: IntoReadonly {
    /// Borrow of the fetch of this query.
    type ReadonlyRef<'borrow>: Copy;

    /// Borrows the fetcher of this query as a readonly fetcher.
    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow>;

    /// Fetches data of the entity from the borrow of the fetcher.
    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        entity: Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>>;
}
