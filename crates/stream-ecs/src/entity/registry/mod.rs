//! Utilities for entity management.

pub use self::error::NotPresentError;

use super::Entity;

pub mod array;

mod error;

/// Entity registry of the world.
///
/// This trait represents type of container for unique entities of the current world.
/// Result of using entities which was created in another world is unspecified
/// and can vary from false-positives to errors and even panics.
pub trait Registry {
    /// Creates new entity which is registered in the registry.
    ///
    /// This method can reuse indices from destroyed entities,
    /// but resulting key should be unique.
    fn create(&mut self) -> Entity;

    /// Checks if the registry contains provided entity.
    fn contains(&self, entity: Entity) -> bool;

    /// Destroys previously created entity.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity
    /// was destroyed earlier or was not created in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// Note that provided entity will be removed from the registry.
    fn destroy(&mut self, entity: Entity) -> Result<(), NotPresentError>;

    /// Returns count of currently alive entities.
    fn len(&self) -> usize;

    /// Checks if the registry contains no alive entities.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the registry, destroying all the entities in it.
    fn clear(&mut self);

    /// Type of iterator of alive entities created by the registry.
    type Iter<'me>: Iterator<Item = Entity>
    where
        Self: 'me;

    /// Returns an iterator of alive entities created by the registry.
    fn iter(&self) -> Self::Iter<'_>;
}

/// Extension of entity registry which allows to implement fallible operations for the registry.
pub trait TryRegistry: Registry {
    /// The type of error which can be returned on failure.
    type Err;

    /// Tries to create new entity which is registered in the registry.
    ///
    /// This method can reuse indices from destroyed entities,
    /// but resulting key should be unique.
    ///
    /// # Errors
    ///
    /// This function will return an error if the registry will fail to create new entity.
    /// Conditions of failure are provided by implementation of the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`create`][Registry::create()] method.
    fn try_create(&mut self) -> Result<Entity, Self::Err>;
}
