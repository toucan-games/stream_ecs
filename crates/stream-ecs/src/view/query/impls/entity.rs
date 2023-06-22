use crate::{
    component::registry::Registry as Components,
    entity::Entity,
    view::query::{IntoReadonly, Query, ReadonlyQuery},
};

impl Query for Entity {
    type Item<'item> = Entity;

    type Fetch<'fetch> = ();

    fn new_fetch<C>(_: &mut C) -> Option<Self::Fetch<'_>>
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

impl IntoReadonly for Entity {
    type Readonly = Self;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        fetch
    }
}

impl ReadonlyQuery for Entity {
    fn new_readonly_fetch<C>(_: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(())
    }

    fn readonly_fetch<'fetch>(
        _fetch: &Self::Fetch<'fetch>,
        entity: Entity,
    ) -> Option<Self::Item<'fetch>> {
        Some(entity)
    }
}
