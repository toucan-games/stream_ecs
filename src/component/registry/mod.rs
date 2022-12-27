//! Component registry utilities of ECS.

use super::Component;

/// Component registry of the world.
///
/// This trait represents type of container for storages.
/// Type of storage is determined by the type of component
/// (see [`Component::Storage`][component_storage] associated type).
///
/// [component_storage]: super::Component::Storage
pub trait Registry: Send + Sync {
    /// Registers the component with component storage.
    ///
    /// Provided storage will be stored in the registry and can be retrieved
    /// by [`storage`][storage] or [`storage_mut`][storage_mut] methods.
    ///
    /// [storage]: Registry::storage()
    /// [storage_mut]: Registry::storage_mut()
    fn register<C>(&mut self, storage: C::Storage)
    where
        C: Component;

    /// Checks if the component was previously registered in the registry.
    fn registered<C>(&self) -> bool
    where
        C: Component;

    /// Unregisters the component, removing it from the registry.
    ///
    /// Storage provided in [`register`][register] method will be removed
    /// from the registry entirely.
    ///
    /// [register]: Registry::register()
    fn unregister<C>(&mut self)
    where
        C: Component;

    /// Clears the registry, removing all component storages in it.
    fn clear(&mut self);

    /// Returns count of component storages which are stored in the registry.
    fn len(&self) -> usize;

    /// Checks if the registry is empty, or has no component storages.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieves a reference to the storage of registered component.
    /// Returns [`None`] if provided component type was not registered.
    fn storage<C>(&self) -> Option<&C::Storage>
    where
        C: Component;

    /// Retrieves a mutable reference to the storage of registered component.
    /// Returns [`None`] if provided component type was not registered.
    fn storage_mut<C>(&mut self) -> Option<&mut C::Storage>
    where
        C: Component;
}
