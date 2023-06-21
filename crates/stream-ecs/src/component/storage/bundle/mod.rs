//! Provides utilities for component storage bundles — heterogenous collections of component storages.

use hlist::ops::Here;

use crate::{
    component::{
        bundle::Bundle as ComponentBundle,
        registry::{
            Registry as Components, RegistryMut as ComponentsMut,
            TryRegistryMut as TryComponentsMut,
        },
    },
    entity::Entity,
};

mod impls;

/// Collection of component storages that can be registered one after another.
///
/// This trait is implemented for all of storages since they can be registered and unregistered trivially.
/// Also it is implemented for heterogenous lists of storages of any size (but not for an empty one).
pub trait Bundle: Sized + 'static {
    /// Component bundle associated with this bundle.
    type Items: ComponentBundle<Storages = Self>;

    /// Registers component bundle in the component registry with provided storage bundle.
    ///
    /// Returns previous bundle data of the component bundle registered earlier.
    /// Returns [`None`] if there was no bundle registered or some of bundle parts are missing.
    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: ComponentsMut;

    /// Unregisters component bundle from the component registry.
    ///
    /// Returns previous bundle data of the component bundle registered earlier.
    /// Returns [`None`] if there was no bundle registered or some of bundle parts are missing.
    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: ComponentsMut;

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
        C: TryComponentsMut;
}

/// Extension of bundle which allows to get a reference to a storage bundle from the registry.
pub trait GetBundle: Bundle {
    /// Type of a reference to the bundle to retrieve from the component registry.
    type Ref<'components>;

    /// Retrieves a reference to the storage bundle which is registered in provided component registry.
    /// Returns [`None`] if provided component registry does not have some bundle storage.
    fn get<C>(components: &C) -> Option<Self::Ref<'_>>
    where
        C: Components;
}

/// Extension of bundle which allows to get a *mutable* reference to a storage bundle from the registry.
pub trait GetBundleMut: Bundle {
    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'components>;

    /// Retrieves a mutable reference to the storage bundle which is registered in provided component registry.
    /// Returns [`None`] if provided component registry does not have some bundle storage.
    fn get_mut<C>(components: &mut C) -> Option<Self::RefMut<'_>>
    where
        C: Components;
}

/// Extension of bundle which allows to get a reference to a component bundle from the registry
/// with **strong** guarantee that components of the bundle always exist in the registry.
///
/// Unlike the [`GetBundle`] trait, this trait provides strong guarantee
/// that such bundle always present in the registry.
/// There is no need to return an [`Option`] from provided trait methods.
///
/// Default generic parameter exists here only to work around the lack of specialization in Rust.
/// Generally it does not need to be used in custom trait implementations,
/// but definitely should be used in generic bounds to support all possible implementations.
pub trait ProvideBundle<C, I = Here>: Bundle
where
    C: Components,
{
    /// Type of a reference to the bundle to retrieve from the component registry.
    type Ref<'components>
    where
        C: 'components;

    /// Retrieves a reference to the bundle which is stored in provided component registry.
    fn provide(components: &C) -> Self::Ref<'_>;
}

/// Extension of bundle which allows to get a *mutable* reference to a component bundle from the registry
/// with **strong** guarantee that components of the bundle always exist in the registry.
///
/// Unlike the [`GetBundleMut`] trait, this trait provides strong guarantee
/// that such bundle always present in the registry.
/// There is no need to return an [`Option`] from provided trait methods.
///
/// Default generic parameter exists here only to work around the lack of specialization in Rust.
/// Generally it does not need to be used in custom trait implementations,
/// but definitely should be used in generic bounds to support all possible implementations.
pub trait ProvideBundleMut<C, I = Here>: Bundle
where
    C: Components,
{
    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'components>
    where
        C: 'components;

    /// Retrieves a mutable reference to the bundle which is stored in provided component registry.
    fn provide_mut(components: &mut C) -> Self::RefMut<'_>;
}

/// Extension of bundle which allows to get a reference to the [items](Bundle::Items) of the storage bundle.
pub trait GetItems: Bundle {
    /// Type of a reference to the items to retrieve from this bundle.
    type ItemsRef<'me>
    where
        Self: 'me;

    /// Retrieves a reference to the items (component bundle) of the provided entity in the storage bundle.
    /// Returns [`None`] if the storage bundle does not have some items by provided entity.
    fn items(&self, entity: Entity) -> Option<Self::ItemsRef<'_>>;
}

/// Extension of bundle which allows to get a *mutable* reference to the [items](Bundle::Items) of the storage bundle.
pub trait GetItemsMut: Bundle {
    /// Type of a mutable reference to the items to retrieve from this bundle.
    type ItemsRefMut<'me>
    where
        Self: 'me;

    /// Retrieves a mutable reference to the items (component bundle) of the provided entity in the storage bundle.
    /// Returns [`None`] if the storage bundle does not have some items by provided entity.
    fn items_mut(&mut self, entity: Entity) -> Option<Self::ItemsRefMut<'_>>;
}
