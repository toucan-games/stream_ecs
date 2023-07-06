use crate::entity::{DefaultEntity, Entity};

use super::{
    iter::ViewRefIter,
    query::{AsReadonly, Query},
};

/// Readonly borrow of the view.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub struct ViewRef<'fetch, Q, E = DefaultEntity>
where
    Q: AsReadonly<E>,
    E: Entity,
{
    fetch: Q::ReadonlyRef<'fetch>,
}

impl<'fetch, Q, E> ViewRef<'fetch, Q, E>
where
    Q: AsReadonly<E>,
    E: Entity,
{
    pub(super) fn new(fetch: Q::ReadonlyRef<'fetch>) -> Self {
        Self { fetch }
    }

    /// Checks if provided entity satisfies this query.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn satisfies(&self, entity: E) -> bool {
        let Self { fetch } = *self;
        Q::readonly_ref_satisfies(fetch, entity)
    }

    /// Get items of the query by provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get(&self, entity: E) -> Option<<Q::Readonly as Query<E>>::Item<'fetch>> {
        let Self { fetch } = *self;
        Q::readonly_ref_fetch(fetch, entity)
    }

    /// Turn this view into an iterator of entities and their data.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn iter<I>(&self, entities: I) -> ViewRefIter<'fetch, Q, I::IntoIter>
    where
        I: IntoIterator<Item = E>,
    {
        let Self { fetch } = *self;
        ViewRefIter::new(entities, fetch)
    }
}

impl<'fetch, Q, E> Clone for ViewRef<'fetch, Q, E>
where
    Q: AsReadonly<E>,
    E: Entity,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'fetch, Q, E> Copy for ViewRef<'fetch, Q, E>
where
    Q: AsReadonly<E>,
    E: Entity,
{
}
