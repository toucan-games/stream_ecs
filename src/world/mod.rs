//! Provides utilities for ECS worlds.

use crate::{
    component::{
        bundle::{Bundle, GetBundle},
        registry::Registry as Components,
        Component,
    },
    entity::{
        builder::StateEntityBuilder,
        entry::{EntityEntry, EntityEntryMut},
        error::{NotPresentError, NotPresentResult},
        registry::Registry as Entities,
        Entity,
    },
    resource::{registry::Registry as Resources, Resource},
};

use self::error::EntityResult;

pub mod error;

/// ECS world — storage of [entities](Entity) and all the [data](Component) attached to them.
///
/// Additionally ECS world can store [resources](Resource) — aka singletons in ECS
/// which does not belong to any specific entity.
#[derive(Debug, Default, Clone)]
pub struct World<E, C, R> {
    entities: E,
    components: C,
    resources: R,
}

impl<E, C, R> World<E, C, R> {
    /// Create new world with provided entity, component and resource registry implementations.
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

    /// Creates an empty [entity builder](StateEntityBuilder), which allows to create new entity *lazily*.
    pub fn builder(&mut self) -> StateEntityBuilder<'_, E, C> {
        let Self {
            entities,
            components,
            resources: _,
        } = self;
        StateEntityBuilder::new(entities, components)
    }

    /// Creates an [entry](EntityEntry) for the provided entity.
    /// Returns [`None`] if provided entity was not in the current world.
    pub fn entry(&self, entity: Entity) -> Option<EntityEntry<'_, E, C>> {
        let Self {
            entities,
            components,
            resources: _,
        } = self;
        EntityEntry::new(entity, entities, components)
    }

    /// Creates a mutable [entry](EntityEntryMut) for the provided entity.
    /// Returns [`None`] if provided entity was not in the current world.
    pub fn entry_mut(&mut self, entity: Entity) -> Option<EntityEntryMut<'_, E, C>> {
        let Self {
            entities,
            components,
            resources: _,
        } = self;
        EntityEntryMut::new(entity, entities, components)
    }

    /// Spawns a new entity and returns a corresponding [entry](EntityEntryMut).
    ///
    /// This is considered as the main API for creation of new entities in the world.
    /// If you only need to create new entity, use [`create`][World::create()] method.
    /// If you need to create entity *lazily*, use [`builder`][World::builder()] method.
    pub fn spawn(&mut self) -> EntityEntryMut<'_, E, C> {
        let Self {
            entities,
            components,
            resources: _,
        } = self;
        EntityEntryMut::spawn(entities, components)
    }

    /// Checks if the world contains provided entity.
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Destroys entity which was previously created in the world.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity
    /// was destroyed earlier or was not created in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn destroy(&mut self, entity: Entity) -> NotPresentResult<()> {
        self.entities.destroy(entity)
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
    pub fn is_registered<T>(&self) -> bool
    where
        T: Component,
    {
        self.components.is_registered::<T>()
    }

    /// Unregisters the component from the current world and returns storage of the component.
    /// Returns [`None`] if the component was not registered.
    pub fn unregister<T>(&mut self) -> Option<T::Storage>
    where
        T: Component,
    {
        self.components.unregister::<T>()
    }

    /// Attaches provided bundle to the entity in the world.
    ///
    /// Returns previous bundle data attached to the entity earlier.
    /// Returns [`None`] if there was no bundle attached to the entity or some of bundle components are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the world
    /// or one of bundle components was not registered in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn attach<B>(&mut self, entity: Entity, bundle: B) -> EntityResult<Option<B>>
    where
        B: Bundle,
    {
        if !self.entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::attach(&mut self.components, entity, bundle)?;
        Ok(bundle)
    }

    /// Checks if all components of the bundle are attached to provided entity in the world.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the world
    /// or one of bundle components was not registered in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn is_attached<B>(&self, entity: Entity) -> EntityResult<bool>
    where
        B: Bundle,
    {
        if !self.entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let is_attached = B::is_attached(&self.components, entity)?;
        Ok(is_attached)
    }

    /// Checks if the entity does not contain any component data attached to it.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity
    /// was destroyed earlier or was not created in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn is_entity_empty(&self, entity: Entity) -> NotPresentResult<bool> {
        if !self.entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let is_empty = self.components.is_entity_empty(entity);
        Ok(is_empty)
    }

    /// Returns previous bundle data attached to the entity earlier.
    /// Returns [`None`] if there was no bundle attached to the entity or some of bundle components are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the world
    /// or one of bundle components was not registered in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn remove<B>(&mut self, entity: Entity) -> EntityResult<Option<B>>
    where
        B: Bundle,
    {
        if !self.entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::remove(&mut self.components, entity)?;
        Ok(bundle)
    }

    /// Removes all attached components from the entity which makes the entity empty.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity
    /// was destroyed earlier or was not created in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn remove_all(&mut self, entity: Entity) -> NotPresentResult<()> {
        if !self.entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let unit = self.components.remove_all(entity);
        Ok(unit)
    }

    /// Retrieves a reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have any of bundle components.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the world
    /// or one of bundle components was not registered in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get<B>(&self, entity: Entity) -> EntityResult<Option<B::Ref<'_>>>
    where
        B: GetBundle,
    {
        if !self.entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::get(&self.components, entity)?;
        Ok(bundle)
    }

    /// Retrieves a mutable reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have any of bundle components.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the world
    /// or one of bundle components was not registered in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get_mut<B>(&mut self, entity: Entity) -> EntityResult<Option<B::RefMut<'_>>>
    where
        B: GetBundle,
    {
        if !self.entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::get_mut(&mut self.components, entity)?;
        Ok(bundle)
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
        self.resources.get()
    }

    /// Retrieves a mutable reference to the inserted resource.
    /// Returns [`None`] if the resource was not inserted in the world.
    pub fn get_resource_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Resource,
    {
        self.resources.get_mut()
    }
}