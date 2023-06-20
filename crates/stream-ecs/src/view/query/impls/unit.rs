use crate::{entity::Entity, view::query::Query};

impl Query for () {
    type Item<'a> = ();

    type Fetch<'a> = ();

    fn fetch<'a>(_: &'a mut Self::Fetch<'_>, _: Entity) -> Option<Self::Item<'a>> {
        Some(())
    }
}
