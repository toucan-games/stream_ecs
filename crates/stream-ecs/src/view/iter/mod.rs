//! Utilities for iteration over all data of the query.

use lending_iterator::prelude::*;
use polonius_the_crab::prelude::*;

use crate::{entity::Entity, view::query::Query};

/// Iterator of the view.
pub struct ViewIter<'a, Q, E>
where
    Q: Query,
    E: Iterator<Item = Entity>,
{
    entities: E,
    fetch: Q::Fetch<'a>,
}

#[gat]
impl<'a, Q, E> LendingIterator for ViewIter<'a, Q, E>
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
