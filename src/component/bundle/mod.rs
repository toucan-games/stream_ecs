//! Provides utilities for bundles - collection of components.

use crate::entity::Entity;

use super::{registry::Registry, storage::Storage, Component};

/// Collection of components that can be attached to an entity one after another.
///
/// This trait is implemented for all of components since they can be attached and removed easily.
/// Also it is implemented for tuples with components of size 12 and less.
pub trait Bundle: Copy + Send + Sync + 'static {
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

impl Bundle for () {
    fn attach<R>(_: &mut R, _: Entity, _: Self)
    where
        R: Registry,
    {
    }

    fn remove<R>(_: &mut R, _: Entity)
    where
        R: Registry,
    {
    }

    fn attached<R>(_: &R, _: Entity) -> bool
    where
        R: Registry,
    {
        false
    }
}

macro_rules! bundle {
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
bundle!(A, B, C, D, E, F, G, H, I, J, K, L);
bundle!(A, B, C, D, E, F, G, H, I, J, K);
bundle!(A, B, C, D, E, F, G, H, I, J);
bundle!(A, B, C, D, E, F, G, H, I);
bundle!(A, B, C, D, E, F, G, H);
bundle!(A, B, C, D, E, F, G);
bundle!(A, B, C, D, E, F);
bundle!(A, B, C, D, E);
bundle!(A, B, C, D);
bundle!(A, B, C);
bundle!(A, B);
bundle!(A);
