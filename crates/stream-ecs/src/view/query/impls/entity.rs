use crate::{component::registry::Registry as Components, entity::Entity, view::query::Query};

impl Query for Entity {
    type Item<'item> = Entity;

    type Fetch<'fetch> = ();

    fn new<C>(_: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(())
    }

    fn fetch<'borrow>(
        _fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        Some(entity)
    }
}
