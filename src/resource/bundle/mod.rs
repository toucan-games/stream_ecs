//! Provides utilities for resource bundles — heterogenous collections of resources.

use super::registry::{Registry as Resources, TryRegistry as TryResources};

mod impls;

/// Collection of resources that can be inserted in the registry one after another.
///
/// This trait is implemented for all of resources since they can be inserted and removed trivially.
/// Also it is implemented for heterogenous lists of resources of any size (but not for an empty one).
pub trait Bundle: Sized + Send + Sync + 'static {
    /// Inserts provided resource bundle to the registry.
    ///
    /// Returns previous bundle data inserted in the registry earlier.
    /// Returns [`None`] if there was no bundle inserted in the registry or some of bundle parts are missing.
    fn insert<R>(resources: &mut R, bundle: Self) -> Option<Self>
    where
        R: Resources;

    /// Removes resource bundle from the registry.
    ///
    /// Returns previous bundle data inserted in the registry earlier.
    /// Returns [`None`] if there was no bundle inserted in the registry or some of bundle parts are missing.
    fn remove<R>(resources: &mut R) -> Option<Self>
    where
        R: Resources;

    /// Checks if all resources of the bundle are inserted to provided registry.
    fn contains<R>(resources: &R) -> bool
    where
        R: Resources;
}

/// Extension of bundle which allows to implement fallible operations for the bundle.
pub trait TryBundle: Bundle {
    /// Tries to insert provided resource bundle to the registry.
    ///
    /// Returns previous bundle data inserted in the registry earlier.
    /// Returns [`None`] if there was no bundle inserted in the registry or some of bundle parts are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided registry will fail to insert provided bundle.
    /// Conditions of failure are provided by implementation of the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`insert`][Bundle::insert()] method.
    fn try_insert<R>(resources: &mut R, bundle: Self) -> Result<Option<Self>, R::Err>
    where
        R: TryResources;
}

/// Extension of bundle which allows to get a reference to a resource bundle from the registry.
pub trait GetBundle: Bundle {
    /// Type of a reference to the bundle to retrieve from the resource registry.
    type Ref<'a>
    where
        Self: 'a;

    /// Retrieves a reference to the resource bundle which is stored in provided registry.
    /// Returns [`None`] if provided registry does not have some bundle resource.
    fn get<R>(resources: &R) -> Option<Self::Ref<'_>>
    where
        R: Resources;
}

/// Extension of bundle which allows to get a *mutable* reference to a resource bundle from the registry.
pub trait GetBundleMut: Bundle {
    /// Type of a mutable reference to the bundle to retrieve from the resource registry.
    type RefMut<'a>
    where
        Self: 'a;

    /// Retrieves a mutable reference to the resource bundle which is stored in provided registry.
    /// Returns [`None`] if provided registry does not have some bundle resource.
    fn get_mut<R>(resources: &mut R) -> Option<Self::RefMut<'_>>
    where
        R: Resources;
}
