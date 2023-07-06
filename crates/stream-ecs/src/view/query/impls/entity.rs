use crate::{
    component::registry::Registry as Components,
    entity::{DefaultEntity, Entity},
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl<Index> Query<DefaultEntity<Index>> for DefaultEntity<Index>
where
    DefaultEntity<Index>: Entity,
{
    type Item<'item> = DefaultEntity<Index>;

    type Fetch<'fetch> = ();

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Self::new_readonly_fetch(components)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: DefaultEntity<Index>,
    ) -> Option<Self::Item<'borrow>> {
        Self::readonly_fetch(fetch, entity)
    }

    fn satisfies(_fetch: &Self::Fetch<'_>, entity: DefaultEntity<Index>) -> bool {
        Self::readonly_ref_satisfies((), entity)
    }
}

impl<Index> IntoReadonly<DefaultEntity<Index>> for DefaultEntity<Index>
where
    DefaultEntity<Index>: Entity,
{
    type Readonly = Self;

    fn into_readonly(
        fetch: Self::Fetch<'_>,
    ) -> <Self::Readonly as Query<DefaultEntity<Index>>>::Fetch<'_> {
        fetch
    }
}

impl<Index> AsReadonly<DefaultEntity<Index>> for DefaultEntity<Index>
where
    DefaultEntity<Index>: Entity,
{
    type ReadonlyRef<'borrow> = ();

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        *fetch
    }

    fn readonly_ref_fetch(
        _fetch: Self::ReadonlyRef<'_>,
        entity: DefaultEntity<Index>,
    ) -> Option<<Self::Readonly as Query<DefaultEntity<Index>>>::Item<'_>> {
        Some(entity)
    }

    fn readonly_ref_satisfies(_: Self::ReadonlyRef<'_>, _: DefaultEntity<Index>) -> bool {
        true
    }
}

impl<Index> ReadonlyQuery<DefaultEntity<Index>> for DefaultEntity<Index>
where
    DefaultEntity<Index>: Entity,
{
    fn new_readonly_fetch<C>(_: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(())
    }

    fn readonly_fetch<'fetch>(
        _fetch: &Self::Fetch<'fetch>,
        entity: DefaultEntity<Index>,
    ) -> Option<Self::Item<'fetch>> {
        Some(entity)
    }
}
