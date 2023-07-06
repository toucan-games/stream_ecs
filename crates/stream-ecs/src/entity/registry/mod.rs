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
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Registry {
    /// Type of entity which is stored in this entity registry.
    type Entity: Entity;

    /// Creates new entity which is registered in the registry.
    ///
    /// This method can reuse indices from destroyed entities,
    /// but resulting key should be unique.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn create(&mut self) -> Self::Entity;

    /// Checks if the registry contains provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn contains(&self, entity: Self::Entity) -> bool;

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
    fn destroy(&mut self, entity: Self::Entity) -> Result<(), NotPresentError<Self::Entity>>;

    /// Returns count of currently alive entities.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn len(&self) -> usize;

    /// Checks if the registry contains no alive entities.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the registry, destroying all the entities in it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn clear(&mut self);

    /// Type of iterator of alive entities created by the registry.
    type Iter<'me>: Iterator<Item = Self::Entity>
    where
        Self: 'me;

    /// Returns an iterator of alive entities created by the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn iter(&self) -> Self::Iter<'_>;
}

/// Extension of entity registry which allows to implement fallible operations for the registry.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
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
    fn try_create(&mut self) -> Result<Self::Entity, Self::Err>;
}
