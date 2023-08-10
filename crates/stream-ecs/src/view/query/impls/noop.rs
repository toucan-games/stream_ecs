use crate::{
    component::registry::Registry as Components,
    entity::Entity,
    view::query::{AsReadonly, IntoReadonly, Noop, Query, ReadonlyQuery},
};

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
