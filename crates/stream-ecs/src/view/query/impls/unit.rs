use crate::{component::registry::Registry as Components, entity::Entity, view::query::Query};

impl Query for () {
    type Item<'item> = ();

    type Fetch<'fetch> = ();

    fn new_fetch<C>(_: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(())
    }

    fn fetch<'borrow>(
        _fetch: &'borrow mut Self::Fetch<'_>,
        _entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        Some(())
    }
}
