use crate::entity::Entity;

use super::{
    iter::ViewRefIter,
    query::{AsReadonly, Query},
};

/// Readonly borrow of the view.
pub struct ViewRef<'fetch, Q>
where
    Q: AsReadonly,
{
    fetch: Q::ReadonlyRef<'fetch>,
}

impl<'fetch, Q> ViewRef<'fetch, Q>
where
    Q: AsReadonly,
{
    pub(super) fn new(fetch: Q::ReadonlyRef<'fetch>) -> Self {
        Self { fetch }
    }

    /// Checks if provided entity satisfies this query.
    pub fn satisfies(&self, entity: Entity) -> bool {
        let Self { fetch } = *self;
        Q::readonly_ref_satisfies(fetch, entity)
    }

    /// Get items of the query by provided entity.
    pub fn get(&self, entity: Entity) -> Option<<Q::Readonly as Query>::Item<'fetch>> {
        let Self { fetch } = *self;
        Q::readonly_ref_fetch(fetch, entity)
    }

    /// Turn this view into an iterator of entities and their data.
    pub fn iter<I>(&self, entities: I) -> ViewRefIter<'fetch, Q, I::IntoIter>
    where
        I: IntoIterator<Item = Entity>,
    {
        let Self { fetch } = *self;
        ViewRefIter::new(entities, fetch)
    }
}

impl<'fetch, Q> Clone for ViewRef<'fetch, Q>
where
    Q: AsReadonly,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'fetch, Q> Copy for ViewRef<'fetch, Q> where Q: AsReadonly {}
