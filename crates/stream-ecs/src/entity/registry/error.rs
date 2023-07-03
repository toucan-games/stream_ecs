use derive_more::Display;

use crate::entity::Entity;

/// The error type which is returned when the entity
/// does not present in the entity registry.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy)]
#[display(fmt = "entity {entity} does not present in the registry")]
pub struct NotPresentError {
    entity: Entity,
}

impl NotPresentError {
    /// Creates new error when the entity does not present in the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }

    /// Returns the entity that was does not present in the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn entity(self) -> Entity {
        self.entity
    }
}
