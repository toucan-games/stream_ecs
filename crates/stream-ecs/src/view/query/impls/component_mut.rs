use crate::{
    component::{registry::Registry as Components, storage::Storage, Component},
    entity::Entity,
    view::query::{AsReadonly, IntoReadonly, Query},
};

impl<C, E> Query<E> for &mut C
where
    C: Component,
    C::Storage: Storage<Entity = E>,
    E: Entity,
{
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
        entity: E,
    ) -> Option<Self::Item<'borrow>> {
        Storage::get_mut(*fetch, entity)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: E) -> bool {
        Self::readonly_ref_satisfies(fetch, entity)
    }
}

impl<'me, C, E> IntoReadonly<E> for &'me mut C
where
    C: Component,
    C::Storage: Storage<Entity = E>,
    E: Entity,
{
    type Readonly = &'me C;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query<E>>::Fetch<'_> {
        fetch
    }
}

impl<C, E> AsReadonly<E> for &mut C
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
