use crate::{entity::Entity, view::query::Query};

impl<T> Query for Option<T>
where
    T: Query,
{
    type Item<'a> = Option<T::Item<'a>>;

    type Fetch<'a> = Option<T::Fetch<'a>>;

    fn fetch<'a>(fetch: &'a mut Self::Fetch<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        let Some(fetch) = fetch else {
            return Some(None);
        };
        let item = T::fetch(fetch, entity);
        Some(item)
    }
}
