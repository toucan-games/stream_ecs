use crate::{
    component::{storage::Storage, Component},
    entity::Entity,
};

use super::Fetch;

/// Fetcher that fetches mutable references of components.
pub struct FetchComponentMut<'a, C>(&'a mut C::Storage)
where
    C: Component;

impl<C> Fetch for FetchComponentMut<'_, C>
where
    C: Component,
{
    type Item<'a> = &'a mut C
    where
        Self: 'a;

    fn fetch(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        let Self(storage) = self;
        storage.get_mut(entity)
    }
}
