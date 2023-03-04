//! Utilities for storages of components in ECS.

use core::any::Any;

use as_any::AsAny;

pub use self::error::TypeMismatchError;

use crate::{component::Component, entity::Entity};

pub mod array;
pub mod bundle;

mod error;

/// Storage of some component type in ECS.
///
/// This trait represents container of components attached to some entities.
/// Furthermore, this trait defines basic operations for such container
/// (for example, to insert or remove component from the storage).
pub trait Storage: Send + Sync + 'static {
    /// Type of component which is stored in this storage.
    type Item: Component<Storage = Self>;

    /// Attaches provided component to the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// Note that this method can reuse existing entities when provided entity
    /// is newer (its generation is greater) than an actual entity with the same index.
    fn attach(&mut self, entity: Entity, component: Self::Item) -> Option<Self::Item>;

    /// Checks if a component is attached to provided entity.
    fn is_attached(&self, entity: Entity) -> bool;

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get(&self, entity: Entity) -> Option<&Self::Item>;

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    /// Removes component from provided entity.
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

    /// Returns an iterator over entity keys
    /// with mutable references of components attached to them.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Extension of storage which allows to implement fallible operations for the storage.
pub trait TryStorage: Storage {
    /// The type of error which can be returned on failure.
    type Err;

    /// Tries to attach provided component to the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// # Errors
    ///
    /// This function will return an error if the storage will fail to attach provided component to the entity.
    /// Conditions of failure are provided by implementation of the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`attach`][Storage::attach()] method.
    fn try_attach(
        &mut self,
        entity: Entity,
        component: Self::Item,
    ) -> Result<Option<Self::Item>, Self::Err>;
}

/// Erased variant of storage of some component type in ECS.
///
/// This trait represents container of components attached to some entities.
/// Furthermore, this trait defines basic operations for such container
/// (for example, to insert or remove component from the storage).
///
/// Compared to [`Storage`] trait, this trait is guaranteed to be object safe, so it can be used as trait object.
/// This trait is implemented for all the storages, so it can be used as trait object for any type of storage.
pub trait ErasedStorage: Send + Sync + AsAny {
    /// Attaches provided component to the entity
    /// only if type of provided component matches the type of component stored in the storage.
    ///
    /// # Errors
    ///
    /// This method will return an error if type of provided component
    /// does not match the type of component stored in the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// Note that this method can reuse existing entities when provided entity
    /// is newer (its generation is greater) than an actual entity with the same index.
    // FIXME: replace result `Ok` type with `Option<impl Component>` when stabilized
    fn attach(&mut self, entity: Entity, component: &dyn Any) -> Result<(), TypeMismatchError>;

    /// Checks if any component is attached to provided entity.
    fn is_attached(&self, entity: Entity) -> bool;

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get(&self, entity: Entity) -> Option<&dyn Any>;

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    fn get_mut(&mut self, entity: Entity) -> Option<&mut dyn Any>;

    /// Removes component from provided entity.
    // FIXME: replace return type with `Option<impl Component>` when stabilized
    fn remove(&mut self, entity: Entity);

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
{
    fn attach(&mut self, entity: Entity, component: &dyn Any) -> Result<(), TypeMismatchError> {
        let Some(component) = component.downcast_ref().copied() else {
            return Err(TypeMismatchError::new::<T::Item>(component));
        };
        Storage::attach(self, entity, component);
        Ok(())
    }

    fn is_attached(&self, entity: Entity) -> bool {
        Storage::is_attached(self, entity)
    }

    fn get(&self, entity: Entity) -> Option<&dyn Any> {
        Storage::get(self, entity).map(AsAny::as_any)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut dyn Any> {
        Storage::get_mut(self, entity).map(AsAny::as_any_mut)
    }

    fn remove(&mut self, entity: Entity) {
        Storage::remove(self, entity);
    }

    fn clear(&mut self) {
        Storage::clear(self)
    }

    fn len(&self) -> usize {
        Storage::len(self)
    }
}
