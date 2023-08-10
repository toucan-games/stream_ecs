use crate::{
    component::registry::Registry as Components,
    entity::{DefaultEntity, Entity},
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl<Index, Generation> Query for DefaultEntity<Index, Generation>
where
    Self: Entity,
{
    type Entity = Self;

    type Item<'item> = Self;

    type Fetch<'fetch> = ();

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Self::new_readonly_fetch(components)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'borrow>> {
        Self::readonly_fetch(fetch, entity)
    }

    fn satisfies(_fetch: &Self::Fetch<'_>, entity: Self::Entity) -> bool {
        Self::readonly_ref_satisfies((), entity)
    }
}

impl<Index, Generation> IntoReadonly for DefaultEntity<Index, Generation>
where
    Self: Entity,
{
    type Readonly = Self;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        fetch
    }
}

impl<Index, Generation> AsReadonly for DefaultEntity<Index, Generation>
where
    Self: Entity,
{
    type ReadonlyRef<'borrow> = ();

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        *fetch
    }

    fn readonly_ref_fetch(
        _fetch: Self::ReadonlyRef<'_>,
        entity: Self::Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>> {
        Some(entity)
    }

    fn readonly_ref_satisfies(_: Self::ReadonlyRef<'_>, _: Self::Entity) -> bool {
        true
    }
}

impl<Index, Generation> ReadonlyQuery for DefaultEntity<Index, Generation>
where
    Self: Entity,
{
    fn new_readonly_fetch<C>(_: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(())
    }

    fn readonly_fetch<'fetch>(
        _fetch: &Self::Fetch<'fetch>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'fetch>> {
        Some(entity)
    }
}
