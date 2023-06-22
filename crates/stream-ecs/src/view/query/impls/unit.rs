use crate::{
    component::registry::Registry as Components,
    entity::Entity,
    view::query::{IntoReadonly, Query, ReadonlyQuery},
};

impl Query for () {
    type Item<'item> = ();

    type Fetch<'fetch> = ();

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
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

impl IntoReadonly for () {
    type Readonly = Self;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        fetch
    }
}

impl ReadonlyQuery for () {
    fn new_readonly_fetch<C>(_: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        Some(())
    }

    fn readonly_fetch<'fetch>(
        _fetch: &Self::Fetch<'fetch>,
        _entity: Entity,
    ) -> Option<Self::Item<'fetch>> {
        Some(())
    }
}
