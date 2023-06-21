//! Utilities for iteration over all data of the query.

use lending_iterator::prelude::*;
use polonius_the_crab::prelude::*;

use crate::{entity::Entity, view::query::Query};

/// Iterator of the view.
pub struct ViewIter<'borrow, 'fetch, Q, E>
where
    Q: Query,
    E: Iterator<Item = Entity>,
{
    entities: E,
    fetch: &'borrow mut Q::Fetch<'fetch>,
}

impl<'borrow, 'fetch, Q, E> ViewIter<'borrow, 'fetch, Q, E>
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
impl<'borrow, 'fetch, Q, E> LendingIterator for ViewIter<'borrow, 'fetch, Q, E>
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
        polonius_loop!(|fetch| -> Option<Q::Item<'polonius>> {
            let Some(entity) = entities.next() else {
                polonius_return!(None);
            };
            let item = Q::fetch(fetch, entity);
            if let Some(item) = item {
                polonius_return!(Some(item));
            }
        });
        None
    }
}
