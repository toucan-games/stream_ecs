use crate::{
    component::{storage::Storage, Component},
    entity::Entity,
};

use super::Fetch;

/// Fetcher that fetches references of components.
pub struct FetchComponent<'a, C>
where
    C: Component,
{
    storage: &'a C::Storage,
}

impl<'_a, C> Fetch for FetchComponent<'_a, C>
where
    C: Component,
{
    type Item<'a> = &'_a C
    where
        Self: 'a;

    fn fetch(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        let Self { storage } = self;
        storage.get(entity)
    }
}
