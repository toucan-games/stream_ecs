use lending_iterator::prelude::*;
use polonius_the_crab::prelude::*;

use crate::{entity::Entity, view::query::Query};

/// Iterator of the view.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub struct ViewIterMut<'borrow, 'fetch, Q, E>
where
    Q: Query<Entity = E::Item>,
    E: Iterator,
    E::Item: Entity,
{
    entities: E,
    fetch: &'borrow mut Q::Fetch<'fetch>,
}

impl<'borrow, 'fetch, Q, E> ViewIterMut<'borrow, 'fetch, Q, E>
where
    Q: Query<Entity = E::Item>,
    E: Iterator,
    E::Item: Entity,
{
    pub(in crate::view) fn new<I>(entities: I, fetch: &'borrow mut Q::Fetch<'fetch>) -> Self
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
    Q: Query<Entity = E::Item>,
    E: Iterator,
    E::Item: Entity,
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
