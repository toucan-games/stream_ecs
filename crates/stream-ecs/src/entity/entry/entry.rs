#![allow(clippy::module_inception)]

use crate::{
    component::{
        bundle::{Bundle, GetBundle, NotRegisteredError},
        registry::Registry as Components,
    },
    entity::{
        registry::{Registry as Entities, TryRegistry as TryEntities},
        Entity,
    },
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
