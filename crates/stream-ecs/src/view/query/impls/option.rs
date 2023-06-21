use crate::{component::registry::Registry as Components, entity::Entity, view::query::Query};

impl<Q> Query for Option<Q>
where
    Q: Query,
{
    type Item<'item> = Option<Q::Item<'item>>;

    type Fetch<'fetch> = Option<Q::Fetch<'fetch>>;

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let new_fetch = Q::new_fetch(components);
        Some(new_fetch)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        let Some(fetch) = fetch else {
            return Some(None);
        };
        let item = Q::fetch(fetch, entity);
        Some(item)
    }
}
