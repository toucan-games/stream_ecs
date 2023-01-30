//! Provides utilities for bundles — heterogenous collections of components.

use crate::entity::Entity;

pub use self::error::{NotRegisteredError, TryBundleError};

use super::{
    registry::Registry as Components,
    storage::{Storage, TryStorage},
    Component,
};

mod error;

/// Collection of components that can be attached to an entity one after another.
///
/// This trait is implemented for all of components since they can be attached and removed trivially.
/// Also it is implemented for tuples with components of size 12 and less (but not for an empty tuple).
pub trait Bundle: Copy + Send + Sync + 'static {
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
        entity: Entity,
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
    fn remove<C>(components: &mut C, entity: Entity) -> Result<Option<Self>, NotRegisteredError>
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
    fn is_attached<C>(components: &C, entity: Entity) -> Result<bool, NotRegisteredError>
    where
        C: Components;
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> Bundle for T
where
    T: Component,
{
    fn attach<C>(
        components: &mut C,
        entity: Entity,
        component: Self,
    ) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.attach(entity, component);
        Ok(component)
    }

    fn remove<C>(components: &mut C, entity: Entity) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.remove(entity);
        Ok(component)
    }

    fn is_attached<C>(components: &C, entity: Entity) -> Result<bool, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let is_attached = storage.is_attached(entity);
        Ok(is_attached)
    }
}

/// Extension of bundle which allows to implement fallible operations for the bundle.
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
        entity: Entity,
        bundle: Self,
    ) -> Result<Option<Self>, TryBundleError<Self::Err>>
    where
        C: Components;
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> TryBundle for T
where
    T: Component,
    T::Storage: TryStorage,
{
    type Err = <T::Storage as TryStorage>::Err;

    fn try_attach<C>(
        components: &mut C,
        entity: Entity,
        component: Self,
    ) -> Result<Option<Self>, TryBundleError<Self::Err>>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            let error = NotRegisteredError::new::<Self>();
            return Err(error.into());
        };
        let component = match storage.try_attach(entity, component) {
            Ok(component) => component,
            Err(err) => return Err(TryBundleError::Storage(err)),
        };
        Ok(component)
    }
}

/// Extension of bundle which allows to get a reference to a bundle from the component registry.
pub trait GetBundle: Bundle {
    /// Type of a reference to the bundle to retrieve from the component registry.
    type Ref<'a>
    where
        Self: 'a;

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
    fn get<C>(components: &C, entity: Entity) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
    where
        C: Components;

    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'a>
    where
        Self: 'a;

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
        entity: Entity,
    ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
    where
        C: Components;
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> GetBundle for T
where
    T: Component,
{
    type Ref<'a> = &'a Self
    where
        Self: 'a;

    fn get<C>(components: &C, entity: Entity) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.get(entity);
        Ok(component)
    }

    type RefMut<'a> = &'a mut Self
    where
        Self: 'a;

    fn get_mut<C>(
        components: &mut C,
        entity: Entity,
    ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.get_mut(entity);
        Ok(component)
    }
}

macro_rules! tuple_length {
    () => {0usize};
    ($head:tt $($tail:tt)*) => {1usize + tuple_length!($($tail)*)};
}

macro_rules! bundle_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> Bundle for ($($types,)*)
        where
            $($types: Component,)*
        {
            #[allow(non_snake_case)]
            fn attach<__C>(components: &mut __C, entity: Entity, bundle: Self) -> Result<Option<Self>, NotRegisteredError>
            where
                __C: Components,
            {
                let _ = Self::is_attached(components, entity)?;
                let ($($types,)*) = bundle;
                $(let $types = $types::attach(components, entity, $types)?;)*
                $(let Some($types) = $types else { return Ok(None); };)*
                let components = Some(($($types,)*));
                Ok(components)
            }

            #[allow(non_snake_case)]
            fn remove<__C>(components: &mut __C, entity: Entity) -> Result<Option<Self>, NotRegisteredError>
            where
                __C: Components,
            {
                let _ = Self::is_attached(components, entity)?;
                $(let $types = $types::remove(components, entity)?;)*
                $(let Some($types) = $types else { return Ok(None); };)*
                let components = Some(($($types,)*));
                Ok(components)
            }

            fn is_attached<__C>(components: &__C, entity: Entity) -> Result<bool, NotRegisteredError>
            where
                __C: Components,
            {
                let is_attached = $($types::is_attached(components, entity)?)&&*;
                Ok(is_attached)
            }
        }
    }
}

// `Bundle` is implemented for tuples of size 12 and less
bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J);
bundle_for_tuple!(A, B, C, D, E, F, G, H, I);
bundle_for_tuple!(A, B, C, D, E, F, G, H);
bundle_for_tuple!(A, B, C, D, E, F, G);
bundle_for_tuple!(A, B, C, D, E, F);
bundle_for_tuple!(A, B, C, D, E);
bundle_for_tuple!(A, B, C, D);
bundle_for_tuple!(A, B, C);
bundle_for_tuple!(A, B);
bundle_for_tuple!(A);

macro_rules! try_bundle_for_tuple {
    ($($types:ident),*; $error_name:ident) => {
        #[doc(hidden)]
        #[derive(Debug, Clone, Copy)]
        pub enum $error_name<$($types),*> {
            $($types($types),)*
        }

        impl<$($types),*> core::fmt::Display for $error_name<$($types),*>
        where
            $($types: core::fmt::Display,)*
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $($error_name::$types(err) => err.fmt(f),)*
                }
            }
        }

        impl<$($types),*> TryBundle for ($($types,)*)
        where
            $(
            $types: Component,
            $types::Storage: TryStorage,
            )*
        {
            type Err = $error_name<$(<$types::Storage as TryStorage>::Err),*>;

            #[allow(non_snake_case)]
            fn try_attach<__C>(
                components: &mut __C,
                entity: Entity,
                bundle: Self,
            ) -> Result<Option<Self>, TryBundleError<Self::Err>>
            where
                __C: Components,
            {
                let _ = Self::is_attached(components, entity)?;
                let ($($types,)*) = bundle;
                $(
                let $types = match $types::try_attach(components, entity, $types) {
                    Ok(bundle) => bundle,
                    Err(err) => match err {
                        TryBundleError::NotRegistered(err) => return Err(err.into()),
                        TryBundleError::Storage(err) => return Err(TryBundleError::Storage($error_name::$types(err))),
                    },
                };
                )*
                $(let Some($types) = $types else { return Ok(None); };)*
                let components = Some(($($types,)*));
                Ok(components)
            }
        }
    }
}

// `TryBundle` is implemented for tuples of size 12 and less
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L; TryBundleErrorTuple12);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K; TryBundleErrorTuple11);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J; TryBundleErrorTuple10);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I; TryBundleErrorTuple9);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H; TryBundleErrorTuple8);
try_bundle_for_tuple!(A, B, C, D, E, F, G; TryBundleErrorTuple7);
try_bundle_for_tuple!(A, B, C, D, E, F; TryBundleErrorTuple6);
try_bundle_for_tuple!(A, B, C, D, E; TryBundleErrorTuple5);
try_bundle_for_tuple!(A, B, C, D; TryBundleErrorTuple4);
try_bundle_for_tuple!(A, B, C; TryBundleErrorTuple3);
try_bundle_for_tuple!(A, B; TryBundleErrorTuple2);
try_bundle_for_tuple!(A; TryBundleErrorTuple1);

macro_rules! get_bundle_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> GetBundle for ($($types,)*)
        where
            $($types: Component,)*
        {
            type Ref<'a> = ($(&'a $types,)*)
            where
                Self: 'a;

            #[allow(non_snake_case)]
            fn get<__C>(components: &__C, entity: Entity) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
            where
                __C: Components,
            {
                $(let $types = $types::get(components, entity)?;)*
                $(let Some($types) = $types else { return Ok(None); };)*
                let components = Some(($($types,)*));
                Ok(components)
            }

            type RefMut<'a> = ($(&'a mut $types,)*)
            where
                Self: 'a;

            #[allow(non_snake_case)]
            fn get_mut<__C>(
                components: &mut __C,
                entity: Entity,
            ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
            where
                __C: Components,
            {
                use core::any::TypeId;

                let mut storages: arrayvec::ArrayVec<_, {tuple_length!($($types)*)}> = components
                    .iter_mut()
                    .filter(|storage| {
                        let type_id = storage.type_id();
                        $(type_id == TypeId::of::<$types>())||*
                    })
                    .collect();
                storages.sort_unstable_by_key(|storage| storage.type_id());

                $(
                let idx = storages
                    .binary_search_by_key(&TypeId::of::<$types>(), |storage| storage.type_id())
                    .ok();
                let Some(idx) = idx else {
                    return Err(NotRegisteredError::new::<$types>());
                };
                let storage = storages
                    .remove(idx)
                    .as_any_mut()
                    .downcast_mut::<$types::Storage>()
                    .expect("storage type casting should succeed because storage was found by TypeId");
                let $types = storage.get_mut(entity);
                )*
                $(let Some($types) = $types else { return Ok(None); };)*

                let components = Some(($($types,)*));
                Ok(components)
            }
        }
    }
}

// `GetBundle` is implemented for tuples of size 12 and less
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H);
get_bundle_for_tuple!(A, B, C, D, E, F, G);
get_bundle_for_tuple!(A, B, C, D, E, F);
get_bundle_for_tuple!(A, B, C, D, E);
get_bundle_for_tuple!(A, B, C, D);
get_bundle_for_tuple!(A, B, C);
get_bundle_for_tuple!(A, B);
get_bundle_for_tuple!(A);
