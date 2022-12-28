//! Utilities for entity management.

use super::Entity;

/// Entity registry of the world.
///
/// This trait represents type of container for unique for the current world entities.
/// Result of using entities which was created in another world is unspecified
/// and can vary from false-positives to errors and even panics.
pub trait Registry: Send + Sync {
    /// Creates new entity which is registered in the registry.
    ///
    /// This method can reuse indices from destroyed entities,
    /// but resulting key should be unique.
    fn create(&mut self) -> Entity;

    /// Checks if the registry contains provided entity.
    fn contains(&self, entity: Entity) -> bool;

    /// Destroys previously created entity.
    ///
    /// Provided entity will be removed from the registry.
    /// If there is no entity to destroy, this method does nothing.
    fn destroy(&mut self, entity: Entity);

    /// Returns count of currently alive entities.
    fn len(&self) -> usize;

    /// Checks if the registry contains no alive entities.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the registry, destroying all the entities in it.
    fn clear(&mut self);

    /// Type of iterator of alive entities created by the registry.
    type Iter: Iterator<Item = Entity>;

    /// Returns an iterator of alive entities created by the registry.
    fn iter(&self) -> Self::Iter;
}
