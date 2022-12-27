//! Resource registry utilities of ECS.

use super::Resource;

/// Resource registry of the world.
///
/// This trait represents type of container for [resources][resource].
///
/// [resource]: super::Resource
pub trait Registry: Send + Sync {
    /// Insert provided resource to the registry.
    ///
    /// Provided resource will be stored in the registry and can be retrieved
    /// by [`resource`][resource] or [`resource_mut`][resource_mut] methods.
    ///
    /// [resource]: Registry::resource()
    /// [resource_mut]: Registry::resource_mut()
    fn insert<R>(&mut self, resource: R)
    where
        R: Resource;

    /// Checks if the resource was previously inserted in the registry.
    fn contains<R>(&self) -> bool
    where
        R: Resource;

    /// Removes the resource from the registry and returns removed resource.
    /// Returns [`None`] if the resource was not inserted.
    ///
    /// Resource provided in [`insert`][insert] method will be removed
    /// from the registry and returned to the user.
    ///
    /// [insert]: Registry::insert()
    fn remove<R>(&mut self) -> Option<R>
    where
        R: Resource;

    /// Clears the registry, removing all resources in it.
    fn clear(&mut self);

    /// Returns count of resources which are stored in the registry.
    fn len(&self) -> usize;

    /// Checks if the registry is empty, or has no resources.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieves a reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the registry.
    fn resource<R>(&self) -> Option<&R>
    where
        R: Resource;

    /// Retrieves a mutable reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the registry.
    fn resource_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource;
}
