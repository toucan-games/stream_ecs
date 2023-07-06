use crate::{
    component::{registry::Registry as Components, storage::Storage, Component},
    entity::Entity,
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl<C, E> Query<E> for &C
where
    C: Component,
    C::Storage: Storage<Entity = E>,
    E: Entity,
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
        entity: E,
    ) -> Option<Self::Item<'borrow>> {
        Self::readonly_fetch(fetch, entity)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: E) -> bool {
        Self::readonly_ref_satisfies(fetch, entity)
    }
}

impl<C, E> IntoReadonly<E> for &C
where
    C: Component,
    C::Storage: Storage<Entity = E>,
    E: Entity,
{
    type Readonly = Self;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query<E>>::Fetch<'_> {
        fetch
    }
}

impl<C, E> AsReadonly<E> for &C
where
    C: Component,
    C::Storage: Storage<Entity = E>,
    E: Entity,
{
    type ReadonlyRef<'borrow> = &'borrow C::Storage;

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        fetch
    }

    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        entity: E,
    ) -> Option<<Self::Readonly as Query<E>>::Item<'_>> {
        Storage::get(fetch, entity)
    }

    fn readonly_ref_satisfies(fetch: Self::ReadonlyRef<'_>, entity: E) -> bool {
        Storage::is_attached(fetch, entity)
    }
}

impl<C, E> ReadonlyQuery<E> for &C
where
    C: Component,
    C::Storage: Storage<Entity = E>,
    E: Entity,
{
    fn new_readonly_fetch<Cs>(components: &Cs) -> Option<Self::Fetch<'_>>
    where
        Cs: Components,
    {
        Components::get::<C>(components)
    }

    fn readonly_fetch<'fetch>(
        fetch: &Self::Fetch<'fetch>,
        entity: E,
    ) -> Option<Self::Item<'fetch>> {
        Storage::get(*fetch, entity)
    }
}
