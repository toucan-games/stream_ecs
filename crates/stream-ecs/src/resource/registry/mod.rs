//! Resource registry utilities of ECS.

use hlist::ops::Here;

use super::{ErasedResource, Resource};

mod impls;

/// Resource registry of the world.
///
/// This trait represents type of container for [resources](Resource).
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Registry {
    /// Checks if the resource was previously inserted in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn contains<R>(&self) -> bool
    where
        R: Resource;

    /// Returns count of resources which are stored in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn len(&self) -> usize;

    /// Checks if the registry is empty, or has no resources.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieves a reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get<R>(&self) -> Option<&R>
    where
        R: Resource;

    /// Retrieves a mutable reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource;

    /// Iterator which returns references of all the resources contained in the registry.
    type Iter<'me>: Iterator<Item = &'me dyn ErasedResource>
    where
        Self: 'me;

    /// Returns an iterator of references of all the resources contained in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn iter(&self) -> Self::Iter<'_>;

    /// Iterator which returns mutable references of all the resources contained in the registry.
    type IterMut<'me>: Iterator<Item = &'me mut dyn ErasedResource>
    where
        Self: 'me;

    /// Returns an iterator of mutable references of all the resources contained in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Extension of resource registry which allows
/// to insert some resource into the registry, resulting in a registry with a new type.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait With: Registry {
    /// Type of the registry with provided type of resource.
    type Output<R>
    where
        R: Resource;

    /// Inserts provided resource into the registry,
    /// resulting in a registry with a new type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn with<R>(self, resource: R) -> Self::Output<R>
    where
        R: Resource;
}

/// Extension of resource registry which allows to modify state of the registry at runtime.
///
/// Implementations of the trait could insert or remove new resources without changing the base type.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait RegistryMut: Registry {
    /// Inserts provided resource into the registry.
    /// Returns previous value of the resource, or [`None`] if the resource was not in the registry.
    ///
    /// Provided resource will be stored in the registry and can be retrieved
    /// by [`get`][get] or [`get_mut`][get_mut] methods.
    ///
    /// [get]: Registry::get()
    /// [get_mut]: Registry::get_mut()
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn insert<R>(&mut self, resource: R) -> Option<R>
    where
        R: Resource;

    /// Removes the resource from the registry and returns removed resource.
    /// Returns [`None`] if the resource was not in the registry.
    ///
    /// Resource provided in [`insert`][insert] method will be removed
    /// from the registry and returned to the user.
    ///
    /// [insert]: RegistryMut::insert()
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn remove<R>(&mut self) -> Option<R>
    where
        R: Resource;

    /// Clears the registry, removing all resources in it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn clear(&mut self);
}

/// Extension of resource registry which allows to implement fallible operations for the registry.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait TryRegistryMut: RegistryMut {
    /// The type of error which can be returned on failure.
    type Err;

    /// Tries to insert provided resource to the registry.
    /// Returns previous value of the resource, or [`None`] if the resource was not in the registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if the registry will fail to insert provided resource.
    /// Conditions of failure are provided by implementation of the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`insert`][insert] method.
    ///
    /// [insert]: RegistryMut::insert()
    fn try_insert<R>(&mut self, resource: R) -> Result<Option<R>, Self::Err>
    where
        R: Resource;
}

/// Extension of resource registry which provides **strong** guarantee that
/// resource provided by generic type parameter always exists in the container.
///
/// Unlike the [`Registry`] trait, this trait provides strong guarantee
/// that such resource always present in the implementation.
/// There is no need to return an [`Option`] from provided trait methods.
///
/// Default generic parameter exists here only to work around the lack of specialization in Rust.
/// Generally it does not need to be used in custom trait implementations,
/// but definitely should be used in generic bounds to support all possible implementations.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Provider<R, I = Here>: Registry
where
    R: Resource,
{
    /// Retrieves a reference to the resource of provided type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn provide(&self) -> &R {
        let Some(resource) = self.get() else {
            let type_name = core::any::type_name::<R>();
            panic!(r#"resource of type "{type_name}" should exist by trait definition"#)
        };
        resource
    }

    /// Retrieves a mutable reference to the resource of provided type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn provide_mut(&mut self) -> &mut R {
        let Some(resource) = self.get_mut() else {
            let type_name = core::any::type_name::<R>();
            panic!(r#"resource of type "{type_name}" should exist by trait definition"#)
        };
        resource
    }
}
