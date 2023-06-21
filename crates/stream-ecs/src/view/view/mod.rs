#![allow(clippy::module_inception)]

use crate::{entity::Entity, view::query::Query};

/// View of entities and their components.
pub struct View<'a, Q>
where
    Q: Query,
{
    fetch: Q::Fetch<'a>,
}

impl<'a, Q> View<'a, Q>
where
    Q: Query,
{
    /// Get items of the query by provided entity.
    pub fn get_mut(&mut self, entity: Entity) -> Option<Q::Item<'_>> {
        let Self { fetch } = self;
        Q::fetch(fetch, entity)
    }
}
