//! Provides utilities for bundles — heterogenous collections of components.

use hlist::ops::Here;

pub use self::error::{NotRegisteredError, TryBundleError};

use super::{registry::Registry as Components, storage::bundle::Bundle as StorageBundle};

mod error;
mod impls;

/// Collection of components that can be attached to an entity one after another.
///
/// This trait is implemented for all of components since they can be attached and removed trivially.
/// Also it is implemented for heterogenous lists of components of any size (but not for an empty one).
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Bundle: Copy + 'static {
    /// Storage bundle associated with this component bundle.
    type Storages: StorageBundle<Items = Self>;

    /// Attaches provided bundle to the entity.
    ///
    /// Returns previous bundle data attached to the entity earlier.
    /// Returns [`None`] if there was no bundle attached to the entity or some of bundle components are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of bundle components
    /// was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn attach<C>(
        components: &mut C,
        entity: <Self::Storages as StorageBundle>::Entity,
        bundle: Self,
    ) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components;

    /// Removes components of the bundle from the entity.
    ///
    /// Returns previous bundle data attached to the entity earlier.
    /// Returns [`None`] if there was no bundle attached to the entity or some of bundle components are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of bundle components
    /// was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn remove<C>(
        components: &mut C,
        entity: <Self::Storages as StorageBundle>::Entity,
    ) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components;

    /// Checks if all components of the bundle are attached to provided entity.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of bundle components
    /// was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_attached<C>(
        components: &C,
        entity: <Self::Storages as StorageBundle>::Entity,
    ) -> Result<bool, NotRegisteredError>
    where
        C: Components;
}

/// Extension of bundle which allows to implement fallible operations for the bundle.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait TryBundle: Bundle {
    /// The type of error which can be returned on failure.
    type Err;

    /// Tries to attach provided bundle to the entity.
    ///
    /// Returns previous bundle data attached to the entity earlier.
    /// Returns [`None`] if there was no bundle attached to the entity or some of bundle components are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of bundle components was not registered in the component registry
    /// or storage of some component will fail to attach it to the entity.
    /// Conditions of failure are provided by implementation of the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`attach`][Bundle::attach()] method.
    fn try_attach<C>(
        components: &mut C,
        entity: <Self::Storages as StorageBundle>::Entity,
        bundle: Self,
    ) -> Result<Option<Self>, TryBundleError<Self::Err>>
    where
        C: Components;
}

/// Extension of bundle which allows to get a reference to a bundle from the component registry.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait GetBundle: Bundle {
    /// Type of a reference to the bundle to retrieve from the component registry.
    type Ref<'components>;

    /// Retrieves a reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have some bundle component.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of bundle components
    /// was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get<C>(
        components: &C,
        entity: <Self::Storages as StorageBundle>::Entity,
    ) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
    where
        C: Components;
}

/// Extension of bundle which allows to get a **mutable** reference to a bundle from the component registry.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait GetBundleMut: Bundle {
    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'components>;

    /// Retrieves a mutable reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have some bundle component.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of bundle components
    /// was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get_mut<C>(
        components: &mut C,
        entity: <Self::Storages as StorageBundle>::Entity,
    ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
    where
        C: Components;
}

/// Extension of bundle which allows to get a reference to a bundle from the component registry
/// with **strong** guarantee that components of the storage bundle always exist in the registry.
///
/// Unlike the [`GetBundle`] trait, this trait provides strong guarantee
/// that such bundle always present in the registry.
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
pub trait ProvideBundle<C, I = Here>: Bundle
where
    C: Components,
{
    /// Type of a reference to the bundle to retrieve from the component registry.
    type Ref<'components>
    where
        C: 'components;

    /// Retrieves a reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have some bundle component.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn provide(
        components: &C,
        entity: <Self::Storages as StorageBundle>::Entity,
    ) -> Option<Self::Ref<'_>>;
}

/// Extension of bundle which allows to get a **mutable** reference to a bundle from the component registry
/// with **strong** guarantee that components of the storage bundle always exist in the registry.
///
/// Unlike the [`GetBundleMut`] trait, this trait provides strong guarantee
/// that such bundle always present in the registry.
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
pub trait ProvideBundleMut<C, I = Here>: Bundle
where
    C: Components,
{
    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'components>
    where
        C: 'components;

    /// Retrieves a mutable reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have some bundle component.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn provide_mut(
        components: &mut C,
        entity: <Self::Storages as StorageBundle>::Entity,
    ) -> Option<Self::RefMut<'_>>;
}
