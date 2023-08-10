use core::{hash::Hash, marker::PhantomData};

use crate::{
    component::registry::Registry as Components,
    entity::{DefaultEntity, Entity},
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

/// Empty type of the query.
///
/// All the methods are noop by its nature.
#[derive(Debug)]
pub struct Noop<E = DefaultEntity>(PhantomData<fn() -> E>)
where
    E: Entity;

impl<E> Clone for Noop<E>
where
    E: Entity,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for Noop<E> where E: Entity {}

impl<E> Default for Noop<E>
where
    E: Entity,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<E> PartialEq for Noop<E>
where
    E: Entity,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E> Eq for Noop<E> where E: Entity {}

impl<E> PartialOrd for Noop<E>
where
    E: Entity,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<E> Ord for Noop<E>
where
    E: Entity,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<E> Hash for Noop<E>
where
    E: Entity,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<E> From<()> for Noop<E>
where
    E: Entity,
{
    fn from(_: ()) -> Self {
        Default::default()
    }
}

impl<E> From<Noop<E>> for ()
where
    E: Entity,
{
    fn from(_: Noop<E>) -> Self {
        Default::default()
    }
}

impl<E> Query for Noop<E>
where
    E: Entity,
{
    type Entity = E;

    type Item<'item> = Self;

    type Fetch<'fetch> = Self;

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        <Self as ReadonlyQuery>::new_readonly_fetch(components)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'borrow>> {
        Self::readonly_fetch(fetch, entity)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: Self::Entity) -> bool {
        Self::readonly_ref_satisfies(*fetch, entity)
    }
}

impl<E> IntoReadonly for Noop<E>
where
    E: Entity,
{
    type Readonly = Self;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        fetch
    }
}

impl<E> AsReadonly for Noop<E>
where
    E: Entity,
{
    type ReadonlyRef<'borrow> = Self;

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        *fetch
    }

    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        _entity: Self::Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>> {
        Some(fetch)
    }

    fn readonly_ref_satisfies(_: Self::ReadonlyRef<'_>, _: Self::Entity) -> bool {
        true
    }
}

impl<E> ReadonlyQuery for Noop<E>
where
    E: Entity,
{
    fn new_readonly_fetch<C>(_: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(Default::default())
    }

    fn readonly_fetch<'fetch>(
        _fetch: &Self::Fetch<'fetch>,
        _entity: Self::Entity,
    ) -> Option<Self::Item<'fetch>> {
        Some(Default::default())
    }
}
