use crate::{
    component::{Component, registry::Registry as Components, storage::Storage},
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl<C> Query for &C
where
    C: Component,
{
    type Entity = <C::Storage as Storage>::Entity;

    type Item<'item> = &'item C;

    type Fetch<'fetch> = &'fetch C::Storage;

    fn new_fetch<Cs>(components: &mut Cs) -> Option<Self::Fetch<'_>>
    where
        Cs: Components,
    {
        Self::new_readonly_fetch(components)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'borrow>> {
        Self::readonly_fetch(fetch, entity)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: Self::Entity) -> bool {
        Self::readonly_ref_satisfies(fetch, entity)
    }
}

impl<C> IntoReadonly for &C
where
    C: Component,
{
    type Readonly = Self;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        fetch
    }
}

impl<C> AsReadonly for &C
where
    C: Component,
{
    type ReadonlyRef<'borrow> = &'borrow C::Storage;

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        fetch
    }

    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        entity: Self::Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>> {
        Storage::get(fetch, entity)
    }

    fn readonly_ref_satisfies(fetch: Self::ReadonlyRef<'_>, entity: Self::Entity) -> bool {
        Storage::is_attached(fetch, entity)
    }
}

impl<C> ReadonlyQuery for &C
where
    C: Component,
{
    fn new_readonly_fetch<Cs>(components: &Cs) -> Option<Self::Fetch<'_>>
    where
        Cs: Components,
    {
        Components::get::<C>(components)
    }

    fn readonly_fetch<'fetch>(
        fetch: &Self::Fetch<'fetch>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'fetch>> {
        Storage::get(*fetch, entity)
    }
}
