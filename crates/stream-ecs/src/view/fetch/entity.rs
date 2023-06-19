use crate::entity::Entity;

use super::Fetch;

/// Fetcher that fetches entities.
pub struct FetchEntity;

impl Fetch<'_> for FetchEntity {
    type Item = Entity;
}
