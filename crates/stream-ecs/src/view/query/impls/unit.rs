use crate::{
    component::registry::Registry as Components,
    entity::Entity,
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl Query for () {
    type Item<'item> = ();

    type Fetch<'fetch> = ();

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Self::new_readonly_fetch(components)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        Self::readonly_fetch(fetch, entity)
    }

    fn satisfies(_fetch: &Self::Fetch<'_>, entity: Entity) -> bool {
        Self::readonly_ref_satisfies((), entity)
    }
}

impl IntoReadonly for () {
    type Readonly = Self;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        fetch
    }
}

impl AsReadonly for () {
    type ReadonlyRef<'borrow> = ();

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        *fetch
    }

    fn readonly_ref_fetch(
        _fetch: Self::ReadonlyRef<'_>,
        _entity: Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>> {
        Some(())
    }

    fn readonly_ref_satisfies(_: Self::ReadonlyRef<'_>, _: Entity) -> bool {
        true
    }
}

impl ReadonlyQuery for () {
    fn new_readonly_fetch<C>(_: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(())
    }

    fn readonly_fetch<'fetch>(
        _fetch: &Self::Fetch<'fetch>,
        _entity: Entity,
    ) -> Option<Self::Item<'fetch>> {
        Some(())
    }
}
