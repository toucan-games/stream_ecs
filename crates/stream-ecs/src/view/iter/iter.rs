#![allow(clippy::module_inception)]

use crate::{entity::Entity, view::query::ReadonlyQuery};

/// Iterator for the view of readonly query.
pub struct ViewIter<'borrow, 'fetch, Q, E>
where
    Q: ReadonlyQuery,
    E: Iterator<Item = Entity>,
{
    entities: E,
    fetch: &'borrow Q::Fetch<'fetch>,
}

impl<'borrow, 'fetch, Q, E> ViewIter<'borrow, 'fetch, Q, E>
where
    Q: ReadonlyQuery,
    E: Iterator<Item = Entity>,
{
    pub(in crate::view) fn new<I>(entities: I, fetch: &'borrow Q::Fetch<'fetch>) -> Self
    where
        I: IntoIterator<IntoIter = E>,
    {
        let entities = entities.into_iter();
        Self { entities, fetch }
    }
}

impl<'borrow, 'fetch, Q, E> Iterator for ViewIter<'borrow, 'fetch, Q, E>
where
    Q: ReadonlyQuery,
    E: Iterator<Item = Entity>,
{
    type Item = Q::Item<'fetch>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { entities, fetch } = self;
        let item = loop {
            let entity = entities.next()?;
            let item = Q::readonly_fetch(fetch, entity);
            if let Some(item) = item {
                break item;
            }
        };
        Some(item)
    }
}
