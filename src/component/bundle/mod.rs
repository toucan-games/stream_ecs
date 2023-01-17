//! Provides utilities for bundles - collection of components.

use crate::entity::Entity;

use super::{registry::Registry, storage::Storage, Component};

/// Collection of components that can be attached to an entity one after another.
///
/// This trait is implemented for all of components since they can be attached and removed trivially.
/// Also it is implemented for tuples with components of size 12 and less (but not for an empty tuple).
pub trait Bundle: Send + Sync + 'static {
    /// Attaches provided bundle to the entity, replacing previous components of the bundle, if any.
    fn attach<R>(components: &mut R, entity: Entity, bundle: Self)
    where
        R: Registry;

    /// Removes components of the bundle from the entity, if any.
    fn remove<R>(components: &mut R, entity: Entity)
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
    fn attach<R>(components: &mut R, entity: Entity, component: Self)
    where
        R: Registry,
    {
        if let Some(storage) = components.storage_mut::<T>() {
            storage.attach(entity, component)
        }
    }

    fn remove<R>(components: &mut R, entity: Entity)
    where
        R: Registry,
    {
        if let Some(storage) = components.storage_mut::<T>() {
            storage.remove(entity)
        }
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

macro_rules! impl_bundle_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> Bundle for ($($types,)*)
        where
            $($types: Component,)*
        {
            fn attach<R>(components: &mut R, entity: Entity, bundle: Self)
            where
                R: Registry,
            {
                #[allow(non_snake_case)]
                let ($($types,)*) = bundle;
                $($types::attach(components, entity, $types);)*
            }

            fn remove<R>(components: &mut R, entity: Entity)
            where
                R: Registry,
            {
                $($types::remove(components, entity);)*
            }

            fn attached<R>(components: &R, entity: Entity) -> bool
            where
                R: Registry,
            {
                $($types::attached(components, entity))&&*
            }
        }
    }
}

// `Bundle` is implemented for tuples of size 12 and less
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H);
impl_bundle_for_tuple!(A, B, C, D, E, F, G);
impl_bundle_for_tuple!(A, B, C, D, E, F);
impl_bundle_for_tuple!(A, B, C, D, E);
impl_bundle_for_tuple!(A, B, C, D);
impl_bundle_for_tuple!(A, B, C);
impl_bundle_for_tuple!(A, B);
impl_bundle_for_tuple!(A);
