use core::fmt::Display;

use crate::entity::Entity;

/// The error type which is returned when the entity
/// does not present in the entity registry.
#[derive(Debug, Clone, Copy)]
pub struct NotPresentError {
    entity: Entity,
}

impl NotPresentError {
    /// Creates new error when the entity does not present in the entity registry.
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }

    /// Returns the entity that was does not present in the entity registry.
    pub fn entity(self) -> Entity {
        self.entity
    }
}

impl Display for NotPresentError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let entity = self.entity;
        write!(f, "entity {entity} does not present in the registry")
    }
}
