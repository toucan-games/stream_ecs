//! Utilities for storages of components in ECS.

use core::any::Any;

pub use self::error::{AttachError, ComponentMismatchError, EntityMismatchError};

use crate::{
    component::{Component, ErasedComponent},
    entity::{Entity, ErasedEntity},
};

pub mod array;
pub mod bundle;

mod error;

/// Storage of some component type in ECS.
///
/// This trait represents container of components attached to some entities.
/// Furthermore, this trait defines basic operations for such container
/// (for example, to insert or remove component from the storage).
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Storage: 'static {
    /// Type of component which is stored in this storage.
    type Item: Component<Storage = Self>;

    /// Type of entity which is used to track stored components.
    type Entity: Entity;

    /// Attaches provided component to the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// Note that this method can reuse existing entities when provided entity
    /// is newer (its generation is greater) than an actual entity with the same index.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn attach(&mut self, entity: Self::Entity, component: Self::Item) -> Option<Self::Item>;

    /// Checks if a component is attached to provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_attached(&self, entity: Self::Entity) -> bool;

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get(&self, entity: Self::Entity) -> Option<&Self::Item>;

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get_mut(&mut self, entity: Self::Entity) -> Option<&mut Self::Item>;

    /// Removes component from provided entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn remove(&mut self, entity: Self::Entity) -> Option<Self::Item>;

    /// Clears this storage, destroying all components in it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn clear(&mut self);

    /// Returns count of components which are stored in the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn len(&self) -> usize;

    /// Checks if the storage is empty, or has no components.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterator which returns entity keys
    /// with references of components attached to them.
    type Iter<'me>: Iterator<Item = (Self::Entity, &'me Self::Item)>
    where
        Self: 'me;

    /// Returns an iterator over entity keys
    /// with references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn iter(&self) -> Self::Iter<'_>;

    /// Iterator which returns entity keys
    /// with mutable references of components attached to them.
    type IterMut<'me>: Iterator<Item = (Self::Entity, &'me mut Self::Item)>
    where
        Self: 'me;

    /// Returns an iterator over entity keys
    /// with mutable references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Extension of storage which allows to implement fallible operations for the storage.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
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
        entity: Self::Entity,
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
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait ErasedStorage: Any {
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
    fn attach(
        &mut self,
        entity: &dyn ErasedEntity,
        component: &dyn ErasedComponent,
    ) -> Result<(), AttachError>;

    /// Checks if any component is attached to provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_attached(&self, entity: &dyn ErasedEntity) -> Result<bool, EntityMismatchError>;

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get(
        &self,
        entity: &dyn ErasedEntity,
    ) -> Result<Option<&dyn ErasedComponent>, EntityMismatchError>;

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn get_mut(
        &mut self,
        entity: &dyn ErasedEntity,
    ) -> Result<Option<&mut dyn ErasedComponent>, EntityMismatchError>;

    /// Removes component from provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    // FIXME: replace return type with `Option<impl Component>` when stabilized
    fn remove(&mut self, entity: &dyn ErasedEntity) -> Result<(), EntityMismatchError>;

    /// Clears this storage, destroying all components in it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn clear(&mut self);

    /// Returns count of components which are stored in the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn len(&self) -> usize;

    /// Checks if the storage is empty, or has no components.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> ErasedStorage for T
where
    T: Storage,
{
    fn attach(
        &mut self,
        entity: &dyn ErasedEntity,
        component: &dyn ErasedComponent,
    ) -> Result<(), AttachError> {
        let component = component as &dyn Any;
        let Some(component) = component.downcast_ref().copied() else {
            let error = ComponentMismatchError::new::<_, T::Item>(component);
            return Err(error.into());
        };

        let entity = entity as &dyn Any;
        let Some(entity) = entity.downcast_ref().copied() else {
            let error = EntityMismatchError::new::<_, T::Entity>(entity);
            return Err(error.into());
        };

        let _ = Storage::attach(self, entity, component);
        Ok(())
    }

    fn is_attached(&self, entity: &dyn ErasedEntity) -> Result<bool, EntityMismatchError> {
        let entity = entity as &dyn Any;
        let Some(entity) = entity.downcast_ref().copied() else {
            let error = EntityMismatchError::new::<_, T::Entity>(entity);
            return Err(error);
        };

        let is_attached = Storage::is_attached(self, entity);
        Ok(is_attached)
    }

    fn get(
        &self,
        entity: &dyn ErasedEntity,
    ) -> Result<Option<&dyn ErasedComponent>, EntityMismatchError> {
        let entity = entity as &dyn Any;
        let Some(entity) = entity.downcast_ref().copied() else {
            let error = EntityMismatchError::new::<_, T::Entity>(entity);
            return Err(error);
        };

        let component = Storage::get(self, entity).map(|item| item as _);
        Ok(component)
    }

    fn get_mut(
        &mut self,
        entity: &dyn ErasedEntity,
    ) -> Result<Option<&mut dyn ErasedComponent>, EntityMismatchError> {
        let entity = entity as &dyn Any;
        let Some(entity) = entity.downcast_ref().copied() else {
            let error = EntityMismatchError::new::<_, T::Entity>(entity);
            return Err(error);
        };

        let component = Storage::get_mut(self, entity).map(|item| item as _);
        Ok(component)
    }

    fn remove(&mut self, entity: &dyn ErasedEntity) -> Result<(), EntityMismatchError> {
        let entity = entity as &dyn Any;
        let Some(entity) = entity.downcast_ref().copied() else {
            let error = EntityMismatchError::new::<_, T::Entity>(entity);
            return Err(error);
        };

        let _ = Storage::remove(self, entity);
        Ok(())
    }

    fn clear(&mut self) {
        Storage::clear(self)
    }

    fn len(&self) -> usize {
        Storage::len(self)
    }

    fn is_empty(&self) -> bool {
        Storage::is_empty(self)
    }
}
