//! Component registry utilities of ECS.

use crate::entity::Entity;

use super::{storage::ErasedStorage, Component};

/// Component registry of the world.
///
/// This trait represents type of container for storages.
/// Type of storage is determined by the type of component
/// (see [`Component::Storage`][component_storage] associated type).
///
/// [component_storage]: super::Component::Storage
pub trait Registry: Send + Sync {
    /// Registers the component in the registry with provided component storage.
    /// Returns previous value of the storage, or [`None`] if the component was not registered.
    ///
    /// Provided storage will be stored in the registry and can be retrieved
    /// by [`get`][Registry::get()] or [`get_mut`][Registry::get_mut()] methods.
    fn register<C>(&mut self, storage: C::Storage) -> Option<C::Storage>
    where
        C: Component;

    /// Checks if the component was previously registered in the registry.
    fn is_registered<C>(&self) -> bool
    where
        C: Component;

    /// Unregisters the component from the registry and returns storage of the component.
    /// Returns [`None`] if the component was not registered.
    ///
    /// Storage provided in [`register`][register] method will be removed
    /// from the registry and returned to the user.
    ///
    /// [register]: Registry::register()
    fn unregister<C>(&mut self) -> Option<C::Storage>
    where
        C: Component;

    /// Clears the registry, removing all component storages in it.
    fn clear(&mut self) {
        for storage in self.iter_mut() {
            storage.clear()
        }
    }

    /// Removes all attached components from the entity which makes the entity empty.
    fn remove_all(&mut self, entity: Entity) {
        for storage in self.iter_mut() {
            storage.remove(entity);
        }
    }

    /// Returns count of component storages which are stored in the registry.
    fn len(&self) -> usize;

    /// Checks if the registry is empty, or has no component storages.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the entity does not contain any component data attached to it.
    fn is_entity_empty(&self, entity: Entity) -> bool {
        self.iter().all(|storage| !storage.is_attached(entity))
    }

    /// Retrieves a reference to the storage of registered component.
    /// Returns [`None`] if provided component type was not registered.
    fn get<C>(&self) -> Option<&C::Storage>
    where
        C: Component;

    /// Retrieves a mutable reference to the storage of registered component.
    /// Returns [`None`] if provided component type was not registered.
    fn get_mut<C>(&mut self) -> Option<&mut C::Storage>
    where
        C: Component;

    /// Iterator which returns references of all the storages
    /// for components registered in the registry.
    type Iter<'a>: Iterator<Item = &'a dyn ErasedStorage>
    where
        Self: 'a;

    /// Returns an iterator of references of all the storages
    /// for components registered in the registry.
    fn iter(&self) -> Self::Iter<'_>;

    /// Iterator which returns mutable references of all the storages
    /// for components registered in the registry.
    type IterMut<'a>: Iterator<Item = &'a mut dyn ErasedStorage>
    where
        Self: 'a;

    /// Returns an iterator of mutable references of all the storages
    /// for components registered in the registry.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Extension of component [registry](Registry) which allows to implement fallible operations for the registry.
pub trait TryRegistry: Registry {
    /// The type of error which can be returned on failure.
    type Err;

    /// Tries to register the component in the registry with provided component storage.
    /// Returns previous value of the storage, or [`None`] if the component was not registered.
    ///
    /// # Errors
    ///
    /// This function will return an error if the registry will fail to register provided component.
    /// Conditions of failure are provided by implementation of the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`register`][Registry::register()] method.
    fn try_register<C>(&mut self, storage: C::Storage) -> Result<Option<C::Storage>, Self::Err>
    where
        C: Component;
}
