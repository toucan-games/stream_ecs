//! Provides utilities for component storage bundles — heterogenous collections of component storages.

use crate::component::registry::{Registry as Components, TryRegistry as TryComponents};

mod impls;

/// Collection of component storages that can be registered one after another.
///
/// This trait is implemented for all of storages since they can be registered and unregistered trivially.
/// Also it is implemented for heterogenous lists of storages of any size (but not for an empty one).
pub trait Bundle: Sized + Send + Sync + 'static {
    /// Registers component bundle in the component registry with provided storage bundle.
    ///
    /// Returns previous bundle data of the component bundle registered earlier.
    /// Returns [`None`] if there was no bundle registered or some of bundle parts are missing.
    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: Components;

    /// Unregisters component bundle from the component registry.
    ///
    /// Returns previous bundle data of the component bundle registered earlier.
    /// Returns [`None`] if there was no bundle registered or some of bundle parts are missing.
    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: Components;

    /// Checks if all storages of the bundle are registered in provided component registry.
    fn is_registered<C>(components: &C) -> bool
    where
        C: Components;
}

/// Extension of bundle which allows to implement fallible operations for the bundle.
pub trait TryBundle: Bundle {
    /// Tries to register component bundle in the component registry with provided storage bundle.
    ///
    /// Returns previous bundle data of the component bundle registered earlier.
    /// Returns [`None`] if there was no bundle registered or some of bundle parts are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided registry will fail to register provided component bundle.
    /// Conditions of failure are provided by implementation of the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`register`][Bundle::register()] method.
    fn try_register<C>(components: &mut C, bundle: Self) -> Result<Option<Self>, C::Err>
    where
        C: TryComponents;
}

/// Extension of bundle which allows to get a reference to a storage bundle from the registry.
pub trait GetBundle: Bundle {
    /// Type of a reference to the bundle to retrieve from the component registry.
    type Ref<'a>
    where
        Self: 'a;

    /// Retrieves a reference to the storage bundle which is registered in provided component registry.
    /// Returns [`None`] if provided component registry does not have some bundle storage.
    fn get<C>(components: &C) -> Option<Self::Ref<'_>>
    where
        C: Components;
}

/// Extension of bundle which allows to get a *mutable* reference to a storage bundle from the registry.
pub trait GetBundleMut: Bundle {
    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'a>
    where
        Self: 'a;

    /// Retrieves a mutable reference to the storage bundle which is registered in provided component registry.
    /// Returns [`None`] if provided component registry does not have some bundle storage.
    fn get_mut<C>(components: &mut C) -> Option<Self::RefMut<'_>>
    where
        C: Components;
}
