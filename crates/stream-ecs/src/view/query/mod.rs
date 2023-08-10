//! Utilities for queries of ECS.

pub use self::noop::Noop;

use crate::{component::registry::Registry as Components, entity::Entity};

mod impls;
mod noop;

/// Type of query to be queried from components by view.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Query {
    /// Type of entity by which the query will be fetched.
    type Entity: Entity;

    /// Type of result yielded by the query.
    type Item<'item>;

    /// Type that fetches query item.
    type Fetch<'fetch>;

    /// Creates new fetcher from provided component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components;

    /// Fetches data of the entity from the fetcher.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'borrow>>;

    /// Checks if provided entity satisfies this query.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn satisfies(fetch: &Self::Fetch<'_>, entity: Self::Entity) -> bool;
}

/// Type of query which is readonly, or has no mutable access to data.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait ReadonlyQuery: AsReadonly<Readonly = Self> {
    /// Creates new fetcher from provided component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn new_readonly_fetch<C>(components: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components;

    /// Fetches data of the entity from the fetcher.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn readonly_fetch<'fetch>(
        fetch: &Self::Fetch<'fetch>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'fetch>>;
}

/// Extension of query which allows to convert this query into readonly query.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait IntoReadonly: Query {
    /// Readonly variant of this query.
    type Readonly: ReadonlyQuery<Entity = Self::Entity>;

    /// Converts the fetcher of this query into a readonly fetcher.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_>;
}

/// Extension of query which allows to borrow this query as readonly query.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait AsReadonly: IntoReadonly {
    /// Borrow of the fetch of this query.
    type ReadonlyRef<'borrow>: Copy;

    /// Borrows the fetcher of this query as a readonly fetcher.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow>;

    /// Fetches data of the entity from the borrow of the fetcher.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        entity: Self::Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>>;

    /// Checks if provided entity satisfies readonly variant of this query.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn readonly_ref_satisfies(fetch: Self::ReadonlyRef<'_>, entity: Self::Entity) -> bool;
}
