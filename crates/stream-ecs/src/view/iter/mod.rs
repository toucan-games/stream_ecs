//! Utilities for iteration over all data of the query.

use lending_iterator::prelude::*;
use polonius_the_crab::prelude::*;

use crate::{
    entity::Entity,
    view::query::{AsReadonly, Query, ReadonlyQuery},
};

/// Iterator of the view.
pub struct ViewIterMut<'borrow, 'fetch, Q, E>
where
    Q: Query,
    E: Iterator<Item = Entity>,
{
    entities: E,
    fetch: &'borrow mut Q::Fetch<'fetch>,
}

impl<'borrow, 'fetch, Q, E> ViewIterMut<'borrow, 'fetch, Q, E>
where
    Q: Query,
    E: Iterator<Item = Entity>,
{
    pub(super) fn new<I>(entities: I, fetch: &'borrow mut Q::Fetch<'fetch>) -> Self
    where
        I: IntoIterator<IntoIter = E>,
    {
        let entities = entities.into_iter();
        Self { entities, fetch }
    }
}

#[gat]
impl<'borrow, 'fetch, Q, E> LendingIterator for ViewIterMut<'borrow, 'fetch, Q, E>
where
    Q: Query,
    E: Iterator<Item = Entity>,
{
    type Item<'next> = Q::Item<'next>
    where
        Self: 'next;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let Self { entities, fetch } = self;
        let mut fetch = fetch;
        let item = polonius_loop!(|fetch| -> _, break: Q::Item<'polonius> {
            let Some(entity) = entities.next() else {
                polonius_return!(None);
            };
            let item = Q::fetch(fetch, entity);
            if let Some(item) = item {
                polonius_break_dependent!(item);
            }
        });
        Some(item)
    }
}

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
    pub(super) fn new<I>(entities: I, fetch: &'borrow Q::Fetch<'fetch>) -> Self
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

/// Iterator for the borrow of the view.
pub struct ViewRefIter<'fetch, Q, E>
where
    Q: AsReadonly,
    E: Iterator<Item = Entity>,
{
    entities: E,
    fetch: Q::ReadonlyRef<'fetch>,
}

impl<'fetch, Q, E> ViewRefIter<'fetch, Q, E>
where
    Q: AsReadonly,
    E: Iterator<Item = Entity>,
{
    pub(super) fn new<I>(entities: I, fetch: Q::ReadonlyRef<'fetch>) -> Self
    where
        I: IntoIterator<IntoIter = E>,
    {
        let entities = entities.into_iter();
        Self { entities, fetch }
    }
}

impl<'fetch, Q, E> Iterator for ViewRefIter<'fetch, Q, E>
where
    Q: AsReadonly,
    E: Iterator<Item = Entity>,
{
    type Item = <Q::Readonly as Query>::Item<'fetch>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            ref mut entities,
            fetch,
        } = *self;
        let item = loop {
            let entity = entities.next()?;
            let item = Q::readonly_ref_fetch(fetch, entity);
            if let Some(item) = item {
                break item;
            }
        };
        Some(item)
    }
}
