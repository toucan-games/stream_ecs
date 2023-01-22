//! Resource registry utilities of ECS.

use core::any::Any;

use super::Resource;

/// Resource registry of the world.
///
/// This trait represents type of container for [resources][resource].
///
/// [resource]: super::Resource
pub trait Registry: Send + Sync {
    /// Insert provided resource to the registry.
    /// Returns previous value of the resource, or [`None`] if the resource was not in the registry.
    ///
    /// Provided resource will be stored in the registry and can be retrieved
    /// by [`get`][Registry::get()] or [`get_mut`][Registry::get_mut()] methods.
    fn insert<R>(&mut self, resource: R) -> Option<R>
    where
        R: Resource;

    /// Checks if the resource was previously inserted in the registry.
    fn contains<R>(&self) -> bool
    where
        R: Resource;

    /// Removes the resource from the registry and returns removed resource.
    /// Returns [`None`] if the resource was not in the registry.
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
    fn get<R>(&self) -> Option<&R>
    where
        R: Resource;

    /// Retrieves a mutable reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the registry.
    fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource;

    /// Iterator which returns references of all the resources contained in the registry.
    type Iter<'a>: Iterator<Item = &'a dyn Any>
    where
        Self: 'a;

    /// Returns an iterator of references of all the resources contained in the registry.
    fn iter(&self) -> Self::Iter<'_>;

    /// Iterator which returns mutable references of all the resources contained in the registry.
    type IterMut<'a>: Iterator<Item = &'a mut dyn Any>
    where
        Self: 'a;

    /// Returns an iterator of mutable references of all the resources contained in the registry.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Extension of resource [registry](Registry) which allows to implement fallible operations for the registry.
pub trait TryRegistry: Registry {
    /// The tpe of error which can be returned on failure.
    type Err;

    /// Tries to insert provided resource to the registry.
    /// Returns previous value of the resource, or [`None`] if the resource was not in the registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if the registry failed to insert provided resource.
    /// Conditions of failure are provided by implementation of the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`insert`][Registry::insert()] method.
    fn try_insert<R>(&mut self, resource: R) -> Result<Option<R>, Self::Err>
    where
        R: Resource;
}
