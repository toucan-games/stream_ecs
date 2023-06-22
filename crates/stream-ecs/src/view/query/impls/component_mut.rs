use crate::{
    component::{registry::Registry as Components, storage::Storage, Component},
    entity::Entity,
    view::query::Query,
};

impl<'me, C> Query for &'me mut C
where
    C: Component,
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
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        Storage::get_mut(*fetch, entity)
    }
}
