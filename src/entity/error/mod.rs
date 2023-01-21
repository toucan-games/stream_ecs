//! Utilities for error handling when operating with entities in ECS.

use super::Entity;

/// The result type which is returned when the entity
/// could be missing in the entity registry.
pub type NotPresentResult<T> = Result<T, NotPresentError>;

/// The error type which is returned when the entity
/// does not present in the entity registry.
#[derive(Debug, Clone, Copy)]
pub struct NotPresentError(Entity);

impl NotPresentError {
    /// Creates new error when the entity does not present in the entity registry.
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }

    /// Returns the entity that was does not present in the entity registry.
    pub fn entity(self) -> Entity {
        let Self(entity) = self;
        entity
    }
}

impl core::fmt::Display for NotPresentError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let entity = self.entity();
        write!(f, "Entity {entity} does not present in the registry")
    }
}
