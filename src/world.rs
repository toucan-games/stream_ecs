//! Provides utilities for ECS worlds.

use crate::{
    component::{bundle::Bundle, registry::Registry as Components, storage::Storage, Component},
    entity::{builder::StatefulEntityBuilder, registry::Registry as Entities, Entity},
    resource::{registry::Registry as Resources, Resource},
};

/// ECS world — storage of entities and all the data attached to them.
///
/// Additionally ECS world can store resources — aka singletons in ECS
/// which does not belong to any specific entity.
#[derive(Debug, Default, Clone)]
pub struct World<E, C, R> {
    entities: E,
    components: C,
    resources: R,
}

impl<E, C, R> World<E, C, R> {
    /// Create new world with provided entities, components and resources
    /// registry implementations.
    pub const fn with(entities: E, components: C, resources: R) -> Self {
        Self {
            entities,
            components,
            resources,
        }
    }

    /// Retrieves a reference of the entity registry of the current world.
    pub const fn entities(&self) -> &E {
        &self.entities
    }

    /// Retrieves a mutable reference of the entity registry of the current world.
    pub fn entities_mut(&mut self) -> &mut E {
        &mut self.entities
    }

    /// Retrieves a reference of the component registry of the current world.
    pub const fn components(&self) -> &C {
        &self.components
    }

    /// Retrieves a mutable reference of the component registry of the current world.
    pub fn components_mut(&mut self) -> &mut C {
        &mut self.components
    }

    /// Retrieves a reference of the resource registry of the current world.
    pub const fn resources(&self) -> &R {
        &self.resources
    }

    /// Retrieves a mutable reference of the resource registry of the current world.
    pub fn resources_mut(&mut self) -> &mut R {
        &mut self.resources
    }

    /// Retrieves references of the entity, component and resource registries of the current world.
    pub fn all(&self) -> (&E, &C, &R) {
        let Self {
            entities,
            components,
            resources,
        } = self;
        (entities, components, resources)
    }

    /// Retrieves mutable references of the entity, component and resource registries of the current world.
    ///
    /// This allows to modify entities, components and resources simultaneously.
    pub fn all_mut(&mut self) -> (&mut E, &mut C, &mut R) {
        let Self {
            entities,
            components,
            resources,
        } = self;
        (entities, components, resources)
    }
}

impl<E, C, R> World<E, C, R>
where
    E: Entities,
    C: Components,
    R: Resources,
{
    /// Creates new empty entity in the current world.
    ///
    /// Newly created entity is empty, so it has no components attached to it.
    pub fn create(&mut self) -> Entity {
        self.entities.create()
    }

    /// Creates new entity in the current world with provided components.
    ///
    /// Newly created entity will have all the components from the provided bundle.
    pub fn create_with<B>(&mut self, bundle: B) -> Entity
    where
        B: Bundle,
    {
        let entity = self.entities.create();
        B::attach(&mut self.components, entity, bundle);
        entity
    }

    /// Creates an empty entity builder to build a new entity with.
    pub fn builder(&mut self) -> StatefulEntityBuilder<'_, E, C> {
        let Self {
            entities,
            components,
            resources: _,
        } = self;
        StatefulEntityBuilder::new(entities, components)
    }

    /// Checks if the world contains provided entity.
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Destroys entity which was previously created in the world.
    pub fn destroy(&mut self, entity: Entity) {
        self.entities.destroy(entity);
    }

    /// Checks if the world contains no entities, components or resources.
    pub fn is_empty(&self) -> bool {
        // skip any checks for components:
        // if there is no entities, component registry can be filled with some data from the removed entities
        self.entities.is_empty() && self.resources.is_empty()
    }

    /// Clears the current world, destroying all entities, their components and all resources of the world.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
        self.resources.clear();
    }

    /// Registers the component in the current world with provided component storage.
    /// Returns previous value of the storage, or [`None`] if the component was not registered.
    pub fn register<T>(&mut self, storage: T::Storage) -> Option<T::Storage>
    where
        T: Component,
    {
        self.components.register::<T>(storage)
    }

    /// Checks if the component was previously registered in the current world.
    pub fn registered<T>(&self) -> bool
    where
        T: Component,
    {
        self.components.registered::<T>()
    }

    /// Unregisters the component from the current world and returns storage of the component.
    /// Returns [`None`] if the component was not registered.
    pub fn unregister<T>(&mut self) -> Option<T::Storage>
    where
        T: Component,
    {
        self.components.unregister::<T>()
    }

    /// Attaches provided component to the entity in the world, replacing previous component data, if any.
    pub fn attach<B>(&mut self, entity: Entity, bundle: B)
    where
        B: Bundle,
    {
        if self.entities.contains(entity) {
            B::attach(&mut self.components, entity, bundle);
        }
    }

    /// Checks if any component is attached to provided entity in the world.
    pub fn attached<B>(&self, entity: Entity) -> bool
    where
        B: Bundle,
    {
        if self.entities.contains(entity) {
            return B::attached(&self.components, entity);
        }
        false
    }

    /// Checks if the entity does not contain any component data attached to it.
    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.components.is_entity_empty(entity)
    }

    /// Removes component from the entity in the world, if any.
    pub fn remove<B>(&mut self, entity: Entity)
    where
        B: Bundle,
    {
        if self.entities.contains(entity) {
            B::remove(&mut self.components, entity);
        }
    }

    /// Removes all attached components from the entity which makes the entity empty.
    pub fn remove_all(&mut self, entity: Entity) {
        self.components.remove_all(entity)
    }

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity doesn't have any component.
    pub fn get<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component,
    {
        let storage = self.components.storage::<T>()?;
        storage.get(entity)
    }

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity doesn't have any component.
    pub fn get_mut<T>(&mut self, entity: Entity) -> Option<&mut T>
    where
        T: Component,
    {
        let storage = self.components.storage_mut::<T>()?;
        storage.get_mut(entity)
    }

    /// Insert provided resource into the current world.
    /// Returns previous value of the resource, or [`None`] if the resource was not in the world.
    pub fn insert_resource<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        self.resources.insert(resource)
    }

    /// Checks if the resource was previously inserted in the current world.
    pub fn contains_resource<T>(&self) -> bool
    where
        T: Resource,
    {
        self.resources.contains::<T>()
    }

    /// Removes the resource from the current world and returns removed resource.
    /// Returns [`None`] if the resource was not in the world.
    pub fn remove_resource<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        self.resources.remove()
    }

    /// Retrieves a reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the world.
    pub fn get_resource<T>(&self) -> Option<&T>
    where
        T: Resource,
    {
        self.resources.resource()
    }

    /// Retrieves a mutable reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the world.
    pub fn get_resource_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Resource,
    {
        self.resources.resource_mut()
    }
}
