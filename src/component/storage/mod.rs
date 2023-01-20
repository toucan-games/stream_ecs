//! Utilities for [storages](Storage) of components in ECS.

use core::{any::Any, mem::ManuallyDrop, ptr::NonNull};

use as_any::AsAny;

use crate::entity::Entity;

use super::Component;

/// Storage of some component type in ECS.
///
/// This trait represents container of components attached to some entities.
/// Furthermore, this trait defines basic operations for such container
/// (for example, to insert or remove component from the storage).
pub trait Storage: Send + Sync + 'static {
    /// Type of component which is stored in this storage.
    type Item: Component;

    /// Attaches provided component to the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    fn attach(&mut self, entity: Entity, component: Self::Item) -> Option<Self::Item>;

    /// Checks if any component is attached to provided entity.
    fn attached(&self, entity: Entity) -> bool;

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get(&self, entity: Entity) -> Option<&Self::Item>;

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    /// Removes component from the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    fn remove(&mut self, entity: Entity) -> Option<Self::Item>;

    /// Clears this storage, destroying all components in it.
    fn clear(&mut self);

    /// Returns count of components which are stored in the storage.
    fn len(&self) -> usize;

    /// Checks if the storage is empty, or has no components.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterator which returns entity keys
    /// with references of components attached to them.
    type Iter<'a>: Iterator<Item = (Entity, &'a Self::Item)>
    where
        Self: 'a;

    /// Returns an iterator over entity keys
    /// with references of components attached to them.
    fn iter(&self) -> Self::Iter<'_>;

    /// Iterator which returns entity keys
    /// with mutable references of components attached to them.
    type IterMut<'a>: Iterator<Item = (Entity, &'a mut Self::Item)>
    where
        Self: 'a;

    /// Returns a mutable iterator over entity keys
    /// with references of components attached to them.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Erased variant of [storage](self::Storage) of some component type in ECS.
///
/// This trait represents container of components attached to some entities.
/// Furthermore, this trait defines basic operations for such container
/// (for example, to insert or remove component from the storage).
///
/// Compared to [`Storage`] trait, this trait is object safe, so it can be used as trait object.
/// This trait is implemented for all the storages, so it can be used as trait object for any storage.
pub trait ErasedStorage: Send + Sync + AsAny {
    /// Attaches provided component to the entity if type of provided component matches the type of item in the storage.
    /// Returns previous component data as erased pointer, or [`None`] if there was no component attached to the entity.
    fn attach(&mut self, entity: Entity, component: &dyn Any) -> Option<NonNull<dyn Any>>;

    /// Checks if any component is attached to provided entity.
    fn attached(&self, entity: Entity) -> bool;

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get(&self, entity: Entity) -> Option<&dyn Any>;

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get_mut(&mut self, entity: Entity) -> Option<&mut dyn Any>;

    /// Removes component from the entity.
    /// Returns previous component data as erased pointer, or [`None`] if there was no component attached to the entity.
    fn remove(&mut self, entity: Entity) -> Option<NonNull<dyn Any>>;

    /// Clears this storage, destroying all components in it.
    fn clear(&mut self);

    /// Returns count of components which are stored in the storage.
    fn len(&self) -> usize;

    /// Checks if the storage is empty, or has no components.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> ErasedStorage for T
where
    T: Storage,
    T::Item: AsAny,
{
    fn attach(&mut self, entity: Entity, component: &dyn Any) -> Option<NonNull<dyn Any>> {
        let component = component.downcast_ref().copied()?;
        let component = Storage::attach(self, entity, component)?;
        let component = ManuallyDrop::new(component);
        Some(NonNull::from(&component))
    }

    fn attached(&self, entity: Entity) -> bool {
        Storage::attached(self, entity)
    }

    fn get(&self, entity: Entity) -> Option<&dyn Any> {
        Storage::get(self, entity).map(AsAny::as_any)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut dyn Any> {
        Storage::get_mut(self, entity).map(AsAny::as_any_mut)
    }

    fn remove(&mut self, entity: Entity) -> Option<NonNull<dyn Any>> {
        let component = Storage::remove(self, entity)?;
        let component = ManuallyDrop::new(component);
        Some(NonNull::from(&component))
    }

    fn clear(&mut self) {
        Storage::clear(self)
    }

    fn len(&self) -> usize {
        Storage::len(self)
    }
}
