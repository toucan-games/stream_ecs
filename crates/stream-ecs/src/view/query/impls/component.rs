use crate::{
    component::{registry::Registry as Components, storage::Storage, Component},
    entity::Entity,
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl<C> Query for &C
where
    C: Component,
{
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
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        Self::readonly_fetch(fetch, entity)
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
        entity: Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>> {
        Storage::get(fetch, entity)
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
        entity: Entity,
    ) -> Option<Self::Item<'fetch>> {
        Storage::get(*fetch, entity)
    }
}
