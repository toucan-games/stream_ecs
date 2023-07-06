use core::fmt::Display;

use derive_more::Display;

use crate::entity::{DefaultEntity, Entity};

/// The error type which is returned when the entity
/// does not present in the entity registry.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy)]
#[display(bound = "E: Display")]
#[display(fmt = "entity {entity} does not present in the registry")]
pub struct NotPresentError<E = DefaultEntity>
where
    E: Entity,
{
    entity: E,
}

impl<E> NotPresentError<E>
where
    E: Entity,
{
    /// Creates new error when the entity does not present in the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new(entity: E) -> Self {
        Self { entity }
    }

    /// Returns the entity that was does not present in the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn entity(self) -> E {
        self.entity
    }
}
