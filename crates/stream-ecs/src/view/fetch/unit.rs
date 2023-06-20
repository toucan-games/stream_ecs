use crate::entity::Entity;

use super::Fetch;

impl Fetch for () {
    type Item<'a> = ()
    where
        Self: 'a;

    fn fetch(&mut self, _: Entity) -> Option<Self::Item<'_>> {
        Some(())
    }
}
