//! Component registry utilities of ECS.

use hlist::ops::Here;

use super::{storage::ErasedStorage, Component};

/// Component registry of the world.
///
/// This trait represents type of container for storages.
/// Type of storage is determined by the type of component
/// (see [`Component::Storage`][component_storage] associated type).
///
/// [component_storage]: super::Component::Storage
pub trait Registry {
    /// Checks if the component was previously registered in the registry.
    fn is_registered<C>(&self) -> bool
    where
        C: Component;

    /// Returns count of component storages which are stored in the registry.
    fn len(&self) -> usize;

    /// Checks if the registry is empty, or has no component storages.
    fn is_empty(&self) -> bool {
        self.len() == 0
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

/// Extension of component registry which allows to modify state of the registry at runtime.
///
/// Implementations of the trait could register or unregister new components without changing the base type.
pub trait RegistryMut: Registry {
    /// Registers the component in the registry with provided component storage.
    /// Returns previous value of the storage, or [`None`] if the component was not registered.
    ///
    /// Provided storage will be stored in the registry and can be retrieved
    /// by [`get`][get] or [`get_mut`][get_mut] methods.
    ///
    /// [get]: Registry::get()
    /// [get_mut]: Registry::get_mut()
    fn register<C>(&mut self, storage: C::Storage) -> Option<C::Storage>
    where
        C: Component;

    /// Unregisters the component from the registry and returns storage of the component.
    /// Returns [`None`] if the component was not registered.
    ///
    /// Storage provided in [`register`][register] method will be removed
    /// from the registry and returned to the user.
    ///
    /// [register]: RegistryMut::register()
    fn unregister<C>(&mut self) -> Option<C::Storage>
    where
        C: Component;

    /// Clears the registry, removing all component storages in it.
    fn clear(&mut self);
}

/// Extension of component registry which allows to implement fallible operations for the registry.
pub trait TryRegistryMut: RegistryMut {
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
    /// This is the fallible version of [`register`][register] method.
    ///
    /// [register]: RegistryMut::register()
    fn try_register<C>(&mut self, storage: C::Storage) -> Result<Option<C::Storage>, Self::Err>
    where
        C: Component;
}

/// Extension of component registry which provides strong guarantee that component
/// provided by generic type parameter is always registered in the container.
///
/// Unlike the [`Registry`] trait, this trait provides strong guarantee
/// that such component is always registered in the implementation.
/// There is no need to return an [`Option`] from provided trait methods.
///
/// Default generic parameter exists here only to work around the lack of specialization in Rust.
/// Generally it does not need to be used in custom trait implementations.
pub trait Provider<C, I = Here>: Registry
where
    C: Component,
{
    /// Retrieves a reference to the storage of provided component.
    fn provide(&self) -> &C::Storage {
        self.get::<C>()
            .expect("component should be registered by trait definition")
    }

    /// Retrieves a mutable reference to the storage of provided component type.
    fn provide_mut(&mut self) -> &mut C::Storage {
        self.get_mut::<C>()
            .expect("component should be registered by trait definition")
    }
}
