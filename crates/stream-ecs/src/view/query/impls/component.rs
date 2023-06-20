use crate::{
    component::{storage::Storage, Component},
    entity::Entity,
    view::query::Query,
};

impl<C> Query for &C
where
    C: Component,
{
    type Item<'a> = &'a C;

    type Fetch<'a> = &'a C::Storage;

    fn fetch<'a>(fetch: &'a mut Self::Fetch<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        fetch.get(entity)
    }
}
