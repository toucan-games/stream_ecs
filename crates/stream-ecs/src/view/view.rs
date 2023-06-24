#![allow(clippy::module_inception)]

use crate::{component::registry::Registry as Components, entity::Entity};

use super::{
    iter::{ViewIter, ViewIterMut},
    query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
    view_ref::ViewRef,
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
    /// Creates new view of entities from provided mutable component registry.
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

    /// Checks if provided entity satisfies this query.
    pub fn satisfies(&self, entity: Entity) -> bool {
        let Self { fetch } = self;
        Q::satisfies(fetch, entity)
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
    Q: IntoReadonly,
{
    /// Converts this view into readonly view.
    pub fn into_readonly(self) -> View<'fetch, Q::Readonly> {
        let Self { fetch } = self;
        let fetch = Q::into_readonly(fetch);
        View::from_fetch(fetch)
    }
}

impl<'fetch, Q> View<'fetch, Q>
where
    Q: AsReadonly,
{
    /// Returns a borrow of the view.
    pub fn as_readonly(&self) -> ViewRef<'_, Q> {
        let Self { fetch } = self;
        let fetch = Q::as_readonly(fetch);
        ViewRef::new(fetch)
    }
}

impl<'fetch, Q> View<'fetch, Q>
where
    Q: ReadonlyQuery,
{
    /// Creates new view of entities from provided component registry.
    pub fn new_readonly<C>(components: &'fetch C) -> Option<Self>
    where
        C: Components,
    {
        let fetch = Q::new_readonly_fetch(components)?;
        Some(Self::from_fetch(fetch))
    }

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
