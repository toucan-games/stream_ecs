//! Utilities for [storages](Storage) of components in ECS.

use crate::entity::Entity;

use super::Component;

/// Storage of some component type in ECS.
///
/// This trait represents container of components attached to some entities.
/// Furthermore, this trait defines basic operations for such container
/// (for example, to insert or remove component from the storage).
pub trait Storage: Send + Sync + 'static {
    /// Type of component which is stored in this storage.
    type Item: Component;

    /// Attaches provided component to the entity, replacing previous component data, if any.
    fn attach(&mut self, entity: Entity, component: Self::Item);

    /// Checks if any component is attached to provided entity.
    fn attached(&self, entity: Entity) -> bool;

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity doesn't have any component.
    fn get(&self, entity: Entity) -> Option<&Self::Item>;

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity doesn't have any component.
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    /// Removes component from the entity, if any.
    fn remove(&mut self, entity: Entity);

    /// Clears this storage, destroying all components in it.
    fn clear(&mut self);

    /// Returns count of components which are stored in the storage.
    fn len(&self) -> usize;

    /// Checks if the storage is empty, or has no components.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterator which returns entity keys
    /// with references of components attached to them.
    type Iter<'a>: Iterator<Item = (Entity, &'a Self::Item)>;

    /// Returns an iterator over entity keys
    /// with references of components attached to them.
    fn iter(&self) -> Self::Iter<'_>;

    /// Iterator which returns entity keys
    /// with mutable references of components attached to them.
    type IterMut<'a>: Iterator<Item = (Entity, &'a mut Self::Item)>;

    /// Returns a mutable iterator over entity keys
    /// with references of components attached to them.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}
