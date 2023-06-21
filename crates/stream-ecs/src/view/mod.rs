//! Provides utilities for views of entities and their components in ECS.

// TODO view API

use lending_iterator::prelude::*;
use polonius_the_crab::prelude::*;

use crate::entity::Entity;

use self::query::Query;

pub mod query;

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
