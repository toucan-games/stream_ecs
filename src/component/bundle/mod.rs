//! Provides utilities for bundles - collection of components.

use core::any::TypeId;

use crate::entity::Entity;

use super::{registry::Registry as Components, storage::Storage, Component};

/// Collection of components that can be attached to an entity one after another.
///
/// This trait is implemented for all of components since they can be attached and removed trivially.
/// Also it is implemented for tuples with components of size 12 and less (but not for an empty tuple).
pub trait Bundle: Copy + Send + Sync + 'static {
    /// Attaches provided bundle to the entity.
    /// Returns previous bundle data, or [`None`] if there was no bundle attached to the entity.
    fn attach<C>(components: &mut C, entity: Entity, bundle: Self) -> Option<Self>
    where
        C: Components;

    /// Removes components of the bundle from the entity.
    /// Returns previous bundle data, or [`None`] if there was no bundle attached to the entity.
    fn remove<C>(components: &mut C, entity: Entity) -> Option<Self>
    where
        C: Components;

    /// Checks if all components of the bundle are attached to provided entity.
    fn attached<C>(components: &C, entity: Entity) -> bool
    where
        C: Components;
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> Bundle for T
where
    T: Component,
{
    fn attach<C>(components: &mut C, entity: Entity, component: Self) -> Option<Self>
    where
        C: Components,
    {
        let storage = components.storage_mut::<T>()?;
        storage.attach(entity, component)
    }

    fn remove<C>(components: &mut C, entity: Entity) -> Option<Self>
    where
        C: Components,
    {
        let storage = components.storage_mut::<T>()?;
        storage.remove(entity)
    }

    fn attached<C>(components: &C, entity: Entity) -> bool
    where
        C: Components,
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
    fn get<C>(components: &C, entity: Entity) -> Option<Self::Ref<'_>>
    where
        C: Components;

    /// Type of a mutable reference to the bundle to retrieve from the component registry.
    type RefMut<'a>
    where
        Self: 'a;

    /// Retrieves a mutable reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have any of bundle components.
    fn get_mut<C>(components: &mut C, entity: Entity) -> Option<Self::RefMut<'_>>
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

    fn get<C>(components: &C, entity: Entity) -> Option<Self::Ref<'_>>
    where
        C: Components,
    {
        let storage = components.storage::<T>()?;
        storage.get(entity)
    }

    type RefMut<'a> = &'a mut Self
    where
        Self: 'a;

    fn get_mut<C>(components: &mut C, entity: Entity) -> Option<Self::RefMut<'_>>
    where
        C: Components,
    {
        let storage = components.storage_mut::<T>()?;
        storage.get_mut(entity)
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
            fn attach<_C>(components: &mut _C, entity: Entity, bundle: Self) -> Option<Self>
            where
                _C: Components,
            {
                let ($($types,)*) = bundle;
                $(let $types = $types::attach(components, entity, $types);)*
                Some(($($types?,)*))
            }

            #[allow(non_snake_case)]
            fn remove<_C>(components: &mut _C, entity: Entity) -> Option<Self>
            where
                _C: Components,
            {
                $(let $types = $types::remove(components, entity);)*
                Some(($($types?,)*))
            }

            fn attached<_C>(components: &_C, entity: Entity) -> bool
            where
                _C: Components,
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
            fn get<_C>(components: &_C, entity: Entity) -> Option<Self::Ref<'_>>
            where
                _C: Components,
            {
                $(let $types = $types::get(components, entity)?;)*
                Some(($($types,)*))
            }

            type RefMut<'a> = ($(&'a mut $types,)*)
            where
                Self: 'a;

            #[allow(non_snake_case)]
            fn get_mut<_C>(components: &mut _C, entity: Entity) -> Option<Self::RefMut<'_>>
            where
                _C: Components,
            {
                let mut storages: arrayvec::ArrayVec<_, {tuple_length!($($types)*)}> = components
                    .iter_mut()
                    .filter(|storage| {
                        let type_id = (**storage).type_id();
                        $(type_id == TypeId::of::<$types>())||*
                    })
                    .collect();
                storages.as_mut_slice().sort_unstable_by_key(|storage| (**storage).type_id());

                $(
                let idx = storages
                    .as_slice()
                    .binary_search_by_key(&TypeId::of::<$types>(), |storage| (**storage).type_id())
                    .ok()?;
                let storage = storages.remove(idx).as_any_mut().downcast_mut::<$types::Storage>()?;
                let $types = storage.get_mut(entity)?;
                )*

                Some(($($types,)*))
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
