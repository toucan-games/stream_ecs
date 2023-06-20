use crate::entity::Entity;

use super::Fetch;

/// Fetcher that fetches entities.
pub struct FetchEntity;

impl Fetch for FetchEntity {
    type Item<'a> = Entity
    where
        Self: 'a;

    fn fetch(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        Some(entity)
    }
}
