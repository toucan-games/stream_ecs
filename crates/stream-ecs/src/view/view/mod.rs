#![allow(clippy::module_inception)]

use crate::{component::registry::Registry as Components, entity::Entity};

use super::{
    iter::{ViewIter, ViewIterMut},
    query::{Query, ReadonlyQuery},
};

/// View of entities and their components.
pub struct View<'fetch, Q>
where
    Q: Query,
{
    fetch: Q::Fetch<'fetch>,
}

impl<'fetch, Q> View<'fetch, Q>
where
    Q: Query,
{
    /// Creates new view of entities from provided component registry.
    pub fn new<C>(components: &'fetch mut C) -> Option<Self>
    where
        C: Components,
    {
        let fetch = Q::new_fetch(components)?;
        Some(Self::from_fetch(fetch))
    }

    /// Creates new view from provided fetcher object.
    pub fn from_fetch(fetch: Q::Fetch<'fetch>) -> Self {
        Self { fetch }
    }

    /// Get mutable items of the query by provided entity.
    pub fn get_mut(&mut self, entity: Entity) -> Option<Q::Item<'_>> {
        let Self { fetch } = self;
        Q::fetch(fetch, entity)
    }

    /// Turn this view into a mutable iterator of entities and their data.
    pub fn iter_mut<I>(&mut self, entities: I) -> ViewIterMut<'_, 'fetch, Q, I::IntoIter>
    where
        I: IntoIterator<Item = Entity>,
    {
        let Self { fetch } = self;
        ViewIterMut::new(entities, fetch)
    }
}

impl<'fetch, Q> View<'fetch, Q>
where
    Q: ReadonlyQuery,
{
    /// Get items of the query by provided entity.
    pub fn get(&self, entity: Entity) -> Option<Q::Item<'fetch>> {
        let Self { fetch } = self;
        Q::readonly_fetch(fetch, entity)
    }

    /// Turn this view into an iterator of entities and their data.
    pub fn iter<I>(&self, entities: I) -> ViewIter<'_, 'fetch, Q, I::IntoIter>
    where
        I: IntoIterator<Item = Entity>,
    {
        let Self { fetch } = self;
        ViewIter::new(entities, fetch)
    }
}
