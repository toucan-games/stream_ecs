use crate::{entity::Entity, view::query::Query};

impl Query for Entity {
    type Item<'a> = Entity;

    type Fetch<'a> = ();

    fn fetch<'a>(_: &'a mut Self::Fetch<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(entity)
    }
}
