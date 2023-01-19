//! Provides utilities for bundles - collection of components.

use crate::entity::Entity;

use super::{registry::Registry, storage::Storage, Component};

/// Collection of components that can be attached to an entity one after another.
///
/// This trait is implemented for all of components since they can be attached and removed trivially.
/// Also it is implemented for tuples with components of size 12 and less (but not for an empty tuple).
pub trait Bundle: Copy + Send + Sync + 'static {
    /// Attaches provided bundle to the entity.
    /// Returns previous bundle data, or [`None`] if there was no bundle attached to the entity.
    fn attach<R>(components: &mut R, entity: Entity, bundle: Self) -> Option<Self>
    where
        R: Registry;

    /// Removes components of the bundle from the entity.
    /// Returns previous bundle data, or [`None`] if there was no bundle attached to the entity.
    fn remove<R>(components: &mut R, entity: Entity) -> Option<Self>
    where
        R: Registry;

    /// Checks if all components of the bundle are attached to provided entity.
    fn attached<R>(components: &R, entity: Entity) -> bool
    where
        R: Registry;
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> Bundle for T
where
    T: Component,
{
    fn attach<R>(components: &mut R, entity: Entity, component: Self) -> Option<Self>
    where
        R: Registry,
    {
        let storage = components.storage_mut::<T>()?;
        storage.attach(entity, component)
    }

    fn remove<R>(components: &mut R, entity: Entity) -> Option<Self>
    where
        R: Registry,
    {
        let storage = components.storage_mut::<T>()?;
        storage.remove(entity)
    }

    fn attached<R>(components: &R, entity: Entity) -> bool
    where
        R: Registry,
    {
        let Some(storage) = components.storage::<T>() else {
            return false;
        };
        storage.attached(entity)
    }
}

/// Extension for [bundles](self::Bundle) which allows to get a reference to a bundle from the component registry.
pub trait GetBundle: Bundle {
    /// Type of a reference to the bundle to retrieve from the component registry.
    type Ref<'a>
    where
        Self: 'a;

    /// Retrieves a reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have any of bundle components.
    fn get<R>(components: &R, entity: Entity) -> Option<Self::Ref<'_>>
    where
        R: Registry;

    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'a>
    where
        Self: 'a;

    /// Retrieves a mutable reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have any of bundle components.
    fn get_mut<R>(components: &mut R, entity: Entity) -> Option<Self::RefMut<'_>>
    where
        R: Registry;
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> GetBundle for T
where
    T: Component,
{
    type Ref<'a> = &'a Self
    where
        Self: 'a;

    fn get<R>(components: &R, entity: Entity) -> Option<Self::Ref<'_>>
    where
        R: Registry,
    {
        let storage = components.storage::<T>()?;
        storage.get(entity)
    }

    type RefMut<'a> = &'a mut Self
    where
        Self: 'a;

    fn get_mut<R>(components: &mut R, entity: Entity) -> Option<Self::RefMut<'_>>
    where
        R: Registry,
    {
        let storage = components.storage_mut::<T>()?;
        storage.get_mut(entity)
    }
}

macro_rules! bundle_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> Bundle for ($($types,)*)
        where
            $($types: Component,)*
        {
            #[allow(non_snake_case)]
            fn attach<R>(components: &mut R, entity: Entity, bundle: Self) -> Option<Self>
            where
                R: Registry,
            {
                let ($($types,)*) = bundle;
                $(let $types = $types::attach(components, entity, $types);)*
                Some(($($types?,)*))
            }

            #[allow(non_snake_case)]
            fn remove<R>(components: &mut R, entity: Entity) -> Option<Self>
            where
                R: Registry,
            {
                $(let $types = $types::remove(components, entity);)*
                Some(($($types?,)*))
            }

            fn attached<R>(components: &R, entity: Entity) -> bool
            where
                R: Registry,
            {
                $($types::attached(components, entity))&&*
            }
        }

        impl<$($types),*> GetBundle for ($($types,)*)
        where
            $($types: Component,)*
        {
            type Ref<'a> = ($(&'a $types,)*)
            where
                Self: 'a;

            #[allow(non_snake_case)]
            fn get<R>(components: &R, entity: Entity) -> Option<Self::Ref<'_>>
            where
                R: Registry,
            {
                $(let $types = $types::get(components, entity)?;)*
                Some(($($types,)*))
            }

            type RefMut<'a> = ($(&'a mut $types,)*)
            where
                Self: 'a;

            #[allow(non_snake_case)]
            fn get_mut<R>(_components: &mut R, _entity: Entity) -> Option<Self::RefMut<'_>>
            where
                R: Registry,
            {
                // TODO: find a way to get multiple mutable references from the registry
                // $(let $types = $types::get_mut(components, entity)?;)*
                // Some(($($types,)*))
                todo!("get mutable references from the registry")
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
