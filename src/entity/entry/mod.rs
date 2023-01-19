//! Provides an entry to the entity.

use crate::component::{
    bundle::{Bundle, GetBundle},
    registry::Registry as Components,
};

use super::{registry::Registry as Entities, Entity};

/// Entry of the specific [entity](Entity).
///
/// Use this struct to simplify access to the entity so
/// you don't have to provide it each time to retrieve something:
/// you can do it only once during struct initialization.
pub struct EntityEntry<'state, E, C> {
    entity: Entity,
    entities: &'state E,
    components: &'state C,
}

impl<'state, E, C> EntityEntry<'state, E, C>
where
    E: Entities,
{
    /// Creates new entry of the specific entity.
    /// Returns [`None`] if there was no entity in provided entity registry.
    pub fn new(entity: Entity, entities: &'state E, components: &'state C) -> Option<Self> {
        if entities.contains(entity) {
            let this = Self {
                entity,
                entities,
                components,
            };
            return Some(this);
        }
        None
    }

    /// Creates new entry of newly created entity.
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

impl<'state, E, C> EntityEntry<'state, E, C> {
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

    /// Retrieves references of the underlying entity and component registries.
    pub fn all(&self) -> (&'state E, &'state C) {
        let &Self {
            entity: _,
            entities,
            components,
        } = self;
        (entities, components)
    }
}

impl<'state, E, C> EntityEntry<'state, E, C>
where
    C: Components,
{
    /// Checks if all components of the bundle are attached to the underlying entity.
    pub fn attached<B>(&self) -> bool
    where
        B: Bundle,
    {
        let &Self {
            entity,
            entities: _,
            components,
        } = self;
        B::attached(components, entity)
    }

    /// Retrieves a reference to the bundle which components are attached to the underlying entity.
    /// Returns [`None`] if the underlying entity does not have any of bundle components.
    pub fn get<B>(&self) -> Option<B::Ref<'_>>
    where
        B: GetBundle,
    {
        let &Self {
            entity,
            entities: _,
            components,
        } = self;
        B::get(components, entity)
    }
}

/// Mutable entry of the specific [entity](Entity).
///
/// Use this struct to simplify access to the entity so
/// you don't have to provide it each time to retrieve something:
/// you can do it only once during struct initialization.
pub struct EntityEntryMut<'state, E, C> {
    entity: Entity,
    entities: &'state mut E,
    components: &'state mut C,
}

impl<'state, E, C> EntityEntryMut<'state, E, C>
where
    E: Entities,
{
    /// Creates new mutable entry of the specific entity.
    /// Returns [`None`] if there was no entity in provided entity registry.
    pub fn new(entity: Entity, entities: &'state mut E, components: &'state mut C) -> Option<Self> {
        if entities.contains(entity) {
            let this = Self {
                entity,
                entities,
                components,
            };
            return Some(this);
        }
        None
    }

    /// Creates new mutable entry of newly created entity.
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
            entity,
            entities,
            components: _,
        } = self;
        entities.destroy(entity);
        entity
    }
}

impl<'state, E, C> EntityEntryMut<'state, E, C> {
    /// Returns unique handle of the entity.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Retrieves a reference of the underlying entity registry.
    pub fn entities(&self) -> &E {
        self.entities
    }

    /// Retrieves a mutable reference of the underlying entity registry.
    pub fn entities_mut(&mut self) -> &mut E {
        self.entities
    }

    /// Retrieves a reference of the underlying component registry.
    pub fn components(&self) -> &C {
        self.components
    }

    /// Retrieves a mutable reference of the underlying component registry.
    pub fn components_mut(&mut self) -> &mut C {
        self.components
    }

    /// Retrieves references of the underlying entity and component registries.
    pub fn all(&self) -> (&E, &C) {
        let Self {
            entity: _,
            entities,
            components,
        } = self;
        (entities, components)
    }

    /// Retrieves mutable references of the underlying entity and component registries.
    pub fn all_mut(&mut self) -> (&mut E, &mut C) {
        let Self {
            entity: _,
            entities,
            components,
        } = self;
        (entities, components)
    }
}

impl<'state, E, C> EntityEntryMut<'state, E, C>
where
    C: Components,
{
    /// Attaches provided bundle to the underlying entity,
    /// replacing previous components of the bundle, if any.
    ///
    /// Returns self mutable reference to allow method chaining.
    pub fn attach<B>(&mut self, bundle: B) -> &mut Self
    where
        B: Bundle,
    {
        let entity = self.entity;
        let components = &mut *self.components;
        B::attach(components, entity, bundle);
        self
    }

    /// Checks if all components of the bundle are attached to the underlying entity.
    pub fn attached<B>(&self) -> bool
    where
        B: Bundle,
    {
        let entity = self.entity;
        let components = &*self.components;
        B::attached(components, entity)
    }

    /// Removes components of the bundle from the underlying entity, if any.
    pub fn remove<B>(&mut self)
    where
        B: Bundle,
    {
        let entity = self.entity;
        let components = &mut *self.components;
        B::remove(components, entity);
    }

    /// Retrieves a reference to the bundle which components are attached to the underlying entity.
    /// Returns [`None`] if the underlying entity does not have any of bundle components.
    pub fn get<B>(&self) -> Option<B::Ref<'_>>
    where
        B: GetBundle,
    {
        let entity = self.entity;
        let components = &*self.components;
        B::get(components, entity)
    }

    /// Retrieves a mutable reference to the bundle which components are attached to the underlying entity.
    /// Returns [`None`] if the underlying entity does not have any of bundle components.
    pub fn get_mut<B>(&mut self) -> Option<B::RefMut<'_>>
    where
        B: GetBundle,
    {
        let entity = self.entity;
        let components = &mut *self.components;
        B::get_mut(components, entity)
    }
}
