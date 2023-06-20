use crate::entity::Entity;

use super::Fetch;

/// Fetcher that fetches optional data from the underlying fetcher.
pub struct FetchOption<T>(T)
where
    T: Fetch;

impl<T> Fetch for FetchOption<T>
where
    T: Fetch,
{
    type Item<'a> = Option<T::Item<'a>>
    where
        Self: 'a;

    fn fetch(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        let Self(fetch) = self;
        let item = fetch.fetch(entity);
        Some(item)
    }
}
