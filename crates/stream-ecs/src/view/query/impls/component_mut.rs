use crate::{
    component::{storage::Storage, Component},
    entity::Entity,
    view::query::Query,
};

impl<C> Query for &mut C
where
    C: Component,
{
    type Item<'a> = &'a mut C;

    type Fetch<'a> = &'a mut C::Storage;

    fn fetch<'a>(fetch: &'a mut Self::Fetch<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        fetch.get_mut(entity)
    }
}
