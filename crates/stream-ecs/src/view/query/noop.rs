use core::{hash::Hash, marker::PhantomData};

use crate::entity::{DefaultEntity, Entity};

/// Empty type of the query.
///
/// All the methods are noop by its nature.
#[derive(Debug)]
pub struct Noop<E = DefaultEntity>(PhantomData<fn() -> E>)
where
    E: Entity;

impl<E> Clone for Noop<E>
where
    E: Entity,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for Noop<E> where E: Entity {}

impl<E> Default for Noop<E>
where
    E: Entity,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<E> PartialEq for Noop<E>
where
    E: Entity,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E> Eq for Noop<E> where E: Entity {}

impl<E> PartialOrd for Noop<E>
where
    E: Entity,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<E> Ord for Noop<E>
where
    E: Entity,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<E> Hash for Noop<E>
where
    E: Entity,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<E> From<()> for Noop<E>
where
    E: Entity,
{
    fn from(_: ()) -> Self {
        Default::default()
    }
}

impl<E> From<Noop<E>> for ()
where
    E: Entity,
{
    fn from(_: Noop<E>) -> Self {
        Default::default()
    }
}
