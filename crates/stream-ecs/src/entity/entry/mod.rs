//! Provides an entry to the entity.

use crate::component::{
    bundle::{Bundle, GetBundle, GetBundleMut, NotRegisteredError, TryBundle, TryBundleError},
    registry::Registry as Components,
};

use super::{
    registry::{Registry as Entities, TryRegistry as TryEntities},
    Entity,
};

/// Entry of the specific [entity](Entity).
///
/// Use this struct to simplify access to the entity so
/// you don't have to provide it each time to retrieve something:
/// you can do it only once during struct initialization.
pub struct Entry<'state, E, C> {
    entity: Entity,
    entities: &'state E,
    components: &'state C,
}

impl<'state, E, C> Entry<'state, E, C>
where
    E: Entities,
{
    /// Creates new entry of the specific entity.
    /// Returns [`None`] if there was no entity in provided entity registry.
    pub fn new(entity: Entity, entities: &'state E, components: &'state C) -> Option<Self> {
        if entities.contains(entity) {
            let entry = Self {
                entity,
                entities,
                components,
            };
            return Some(entry);
        }
        None
    }

    /// Creates new entity and an entry for it.
    ///
    /// New entity will be created by provided entity registry.
    pub fn spawn(entities: &'state mut E, components: &'state C) -> Self {
        let entity = entities.create();
        Self {
            entity,
            entities,
            components,
        }
    }
}

impl<'state, E, C> Entry<'state, E, C>
where
    E: TryEntities,
{
    /// Tries to create new entity and an entry for it.
    ///
    /// New entity will be created by provided entity registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity registry will fail to create new entity.
    /// Conditions of failure are provided by implementation of provided entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`spawn`][Entry::spawn()] method.
    pub fn try_spawn(entities: &'state mut E, components: &'state C) -> Result<Self, E::Err> {
        let entity = entities.try_create()?;
        let entry = Self {
            entity,
            entities,
            components,
        };
        Ok(entry)
    }
}

impl<'state, E, C> Entry<'state, E, C> {
    /// Returns unique handle of the entity.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Retrieves a reference of the underlying entity registry.
    pub fn entities(&self) -> &'state E {
        self.entities
    }

    /// Retrieves a reference of the underlying component registry.
    pub fn components(&self) -> &'state C {
        self.components
    }
}

impl<'state, E, C> Entry<'state, E, C>
where
    C: Components,
{
    /// Checks if all components of the bundle are attached to the underlying entity.
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
    pub fn is_attached<B>(&self) -> Result<bool, NotRegisteredError>
    where
        B: Bundle,
    {
        let &Self {
            entity, components, ..
        } = self;
        B::is_attached(components, entity)
    }

    /// Retrieves a reference to the bundle which components are attached to the underlying entity.
    /// Returns [`None`] if the underlying entity does not have some bundle component.
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
    pub fn get<B>(&self) -> Result<Option<B::Ref<'_>>, NotRegisteredError>
    where
        B: GetBundle,
    {
        let &Self {
            entity, components, ..
        } = self;
        B::get(components, entity)
    }
}

/// Mutable entry of the specific [entity](Entity).
///
/// Use this struct to simplify access to the entity so
/// you don't have to provide it each time to retrieve something:
/// you can do it only once during struct initialization.
pub struct EntryMut<'state, E, C> {
    entity: Entity,
    entities: &'state mut E,
    components: &'state mut C,
}

impl<'state, E, C> EntryMut<'state, E, C>
where
    E: Entities,
{
    /// Creates new mutable entry of the specific entity.
    /// Returns [`None`] if there was no entity in provided entity registry.
    pub fn new(entity: Entity, entities: &'state mut E, components: &'state mut C) -> Option<Self> {
        if entities.contains(entity) {
            let entry = Self {
                entity,
                entities,
                components,
            };
            return Some(entry);
        }
        None
    }

    /// Creates new entity and a mutable entry for it.
    ///
    /// New entity will be created by provided entity registry.
    pub fn spawn(entities: &'state mut E, components: &'state mut C) -> Self {
        let entity = entities.create();
        Self {
            entity,
            entities,
            components,
        }
    }

    /// Destroys the underlying entity, returning its handle.
    pub fn destroy(self) -> Entity {
        let Self {
            entity, entities, ..
        } = self;
        entities
            .destroy(entity)
            .expect("entity should present in the registry");
        entity
    }
}

impl<'state, E, C> EntryMut<'state, E, C>
where
    E: TryEntities,
{
    /// Tries to create new entity and a mutable entry for it.
    ///
    /// New entity will be created by provided entity registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity registry will fail to create new entity.
    /// Conditions of failure are provided by implementation of provided entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`spawn`][EntryMut::spawn()] method.
    pub fn try_spawn(entities: &'state mut E, components: &'state mut C) -> Result<Self, E::Err> {
        let entity = entities.try_create()?;
        let entry = Self {
            entity,
            entities,
            components,
        };
        Ok(entry)
    }
}

impl<'state, E, C> EntryMut<'state, E, C> {
    /// Returns unique handle of the entity.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Retrieves a reference of the underlying entity registry.
    pub fn entities(&self) -> &E {
        self.entities
    }

    /// Retrieves a reference of the underlying component registry.
    pub fn components(&self) -> &C {
        self.components
    }
}

impl<'state, E, C> EntryMut<'state, E, C>
where
    C: Components,
{
    /// Attaches provided bundle to the underlying entity.
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
    pub fn attach<B>(&mut self, bundle: B) -> Result<Option<B>, NotRegisteredError>
    where
        B: Bundle,
    {
        let entity = self.entity;
        let components = &mut *self.components;
        B::attach(components, entity, bundle)
    }

    /// Tries to attach provided bundle to the underlying entity.
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
    /// This is the fallible version of [`attach`][EntryMut::attach()] method.
    pub fn try_attach<B>(&mut self, bundle: B) -> Result<Option<B>, TryBundleError<B::Err>>
    where
        B: TryBundle,
    {
        let entity = self.entity;
        let components = &mut *self.components;
        B::try_attach(components, entity, bundle)
    }

    /// Checks if all components of the bundle are attached to the underlying entity.
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
    pub fn is_attached<B>(&self) -> Result<bool, NotRegisteredError>
    where
        B: Bundle,
    {
        let entity = self.entity;
        let components = &*self.components;
        B::is_attached(components, entity)
    }

    /// Removes components of the bundle from the underlying entity.
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
    pub fn remove<B>(&mut self) -> Result<Option<B>, NotRegisteredError>
    where
        B: Bundle,
    {
        let entity = self.entity;
        let components = &mut *self.components;
        B::remove(components, entity)
    }

    /// Retrieves a reference to the bundle which components are attached to the underlying entity.
    /// Returns [`None`] if the underlying entity does not have some bundle component.
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
    pub fn get<B>(&self) -> Result<Option<B::Ref<'_>>, NotRegisteredError>
    where
        B: GetBundle,
    {
        let entity = self.entity;
        let components = &*self.components;
        B::get(components, entity)
    }

    /// Retrieves a mutable reference to the bundle which components are attached to the underlying entity.
    /// Returns [`None`] if the underlying entity does not have some bundle component.
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
    pub fn get_mut<B>(&mut self) -> Result<Option<B::RefMut<'_>>, NotRegisteredError>
    where
        B: GetBundleMut,
    {
        let entity = self.entity;
        let components = &mut *self.components;
        B::get_mut(components, entity)
    }
}
