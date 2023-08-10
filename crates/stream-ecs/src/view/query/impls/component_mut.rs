use crate::{
    component::{registry::Registry as Components, storage::Storage, Component},
    view::query::{AsReadonly, IntoReadonly, Query},
};

impl<C> Query for &mut C
where
    C: Component,
{
    type Entity = <C::Storage as Storage>::Entity;

    type Item<'item> = &'item mut C;

    type Fetch<'fetch> = &'fetch mut C::Storage;

    fn new_fetch<Cs>(components: &mut Cs) -> Option<Self::Fetch<'_>>
    where
        Cs: Components,
    {
        Components::get_mut::<C>(components)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'borrow>> {
        Storage::get_mut(*fetch, entity)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: Self::Entity) -> bool {
        Self::readonly_ref_satisfies(fetch, entity)
    }
}

impl<'me, C> IntoReadonly for &'me mut C
where
    C: Component,
{
    type Readonly = &'me C;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        fetch
    }
}

impl<C> AsReadonly for &mut C
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
