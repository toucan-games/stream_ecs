use crate::{
    component::{registry::Registry as Components, storage::Storage, Component},
    entity::Entity,
    view::query::Query,
};

impl<C> Query for &C
where
    C: Component,
{
    type Item<'item> = &'item C;

    type Fetch<'fetch> = &'fetch C::Storage;

    fn new_fetch<_C>(components: &mut _C) -> Option<Self::Fetch<'_>>
    where
        _C: Components,
    {
        components.get::<C>()
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        Storage::get(*fetch, entity)
    }
}
