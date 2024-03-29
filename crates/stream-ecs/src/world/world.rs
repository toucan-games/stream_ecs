#![allow(clippy::module_inception)]

use crate::{
    component::{
        bundle::{
            Bundle, GetBundle, GetBundleMut, NotRegisteredError, ProvideBundle, ProvideBundleMut,
            TryBundle, TryBundleError,
        },
        registry::{
            Registry as Components, RegistryMut as ComponentsMut,
            TryRegistryMut as TryComponentsMut, With as WithComponents,
        },
        storage::bundle::{Bundle as StorageBundle, TryBundle as StorageTryBundle},
    },
    entity::{
        builder::{TryBuildError, TryEntityBuildError},
        entry::{Entry, EntryMut},
        registry::{NotPresentError, Registry as Entities, TryRegistry as TryEntities},
    },
    resource::{
        bundle::{
            Bundle as ResourceBundle, GetBundle as ResourceGetBundle,
            GetBundleMut as ResourceGetBundleMut, ProvideBundle as ResourceProvideBundle,
            ProvideBundleMut as ResourceProvideBundleMut, TryBundle as ResourceTryBundle,
        },
        registry::{
            Registry as Resources, RegistryMut as ResourcesMut, TryRegistryMut as TryResourcesMut,
            With as WithResources,
        },
    },
    view::query::{Query, ReadonlyQuery},
};

use super::{
    builder::EntityBuilder,
    error::{EntityError, TryAttachError},
    view::View,
};

/// ECS world — storage of [entities](crate::entity::Entity)
/// and all the [data](crate::component::Component) attached to them.
///
/// Additionally ECS world can store [resources](crate::resource::Resource) — aka singletons in ECS
/// which does not belong to any specific entity.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct World<E, C, R> {
    /// Entity registry of the world.
    pub entities: E,
    /// Component registry of the world.
    pub components: C,
    /// Resource registry of the world.
    pub resources: R,
}

impl<E, C, R> World<E, C, R> {
    /// Create new world with provided entity, component and resource registry implementations.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn with(entities: E, components: C, resources: R) -> Self {
        Self {
            entities,
            components,
            resources,
        }
    }

    /// Retrieves a reference of the entity registry of the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn entities(&self) -> &E {
        let Self { entities, .. } = self;
        entities
    }

    /// Retrieves a mutable reference of the entity registry of the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn entities_mut(&mut self) -> &mut E {
        let Self { entities, .. } = self;
        entities
    }

    /// Retrieves a reference of the component registry of the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn components(&self) -> &C {
        let Self { components, .. } = self;
        components
    }

    /// Retrieves a mutable reference of the component registry of the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn components_mut(&mut self) -> &mut C {
        let Self { components, .. } = self;
        components
    }

    /// Retrieves a reference of the resource registry of the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn resources(&self) -> &R {
        let Self { resources, .. } = self;
        resources
    }

    /// Retrieves a mutable reference of the resource registry of the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn resources_mut(&mut self) -> &mut R {
        let Self { resources, .. } = self;
        resources
    }
}

impl<E, C, R> World<E, C, R>
where
    E: Entities,
{
    /// Creates new empty entity in the current world.
    ///
    /// Newly created entity is empty, so it has no components attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn create(&mut self) -> E::Entity {
        let Self { entities, .. } = self;
        entities.create()
    }

    /// Creates an [entry](Entry) for the provided entity.
    /// Returns [`None`] if provided entity was not in the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn entry(&self, entity: E::Entity) -> Option<Entry<'_, E, C>> {
        let Self {
            entities,
            components,
            ..
        } = self;
        Entry::new(entity, entities, components)
    }

    /// Creates a mutable [entry](EntryMut) for the provided entity.
    /// Returns [`None`] if provided entity was not in the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn entry_mut(&mut self, entity: E::Entity) -> Option<EntryMut<'_, E, C>> {
        let Self {
            entities,
            components,
            ..
        } = self;
        EntryMut::new(entity, entities, components)
    }

    /// Spawns a new entity and returns a corresponding [entry](EntryMut).
    ///
    /// This is considered as the main API for creation of new entities in the world.
    /// If you only need to create new entity, use [`create`][World::create()] method.
    /// If you need to create entity *lazily*, use [`builder_from`][World::builder_from()] method.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn spawn(&mut self) -> EntryMut<'_, E, C> {
        let Self {
            entities,
            components,
            ..
        } = self;
        EntryMut::spawn(entities, components)
    }

    /// Checks if the world contains provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn contains(&self, entity: E::Entity) -> bool {
        let Self { entities, .. } = self;
        entities.contains(entity)
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
    pub fn destroy(&mut self, entity: E::Entity) -> Result<(), NotPresentError<E::Entity>> {
        let Self { entities, .. } = self;
        entities.destroy(entity)
    }
}

impl<E, C, R> World<E, C, R>
where
    E: TryEntities,
{
    /// Tries to create new empty entity in the current world.
    ///
    /// Newly created entity is empty, so it has no components attached to it.
    ///
    /// # Errors
    ///
    /// This function will return an error if the world will fail to create new entity.
    /// Conditions of failure are provided by implementation of the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`create`][World::create()] method.
    pub fn try_create(&mut self) -> Result<E::Entity, E::Err> {
        let Self { entities, .. } = self;
        entities.try_create()
    }

    /// Tries to spawn a new entity and return a corresponding [entry](EntryMut).
    ///
    /// # Errors
    ///
    /// This function will return an error if the world will fail to create new entity.
    /// Conditions of failure are provided by implementation of the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`spawn`][World::spawn()] method.
    pub fn try_spawn(&mut self) -> Result<EntryMut<'_, E, C>, E::Err> {
        let Self {
            entities,
            components,
            ..
        } = self;
        EntryMut::try_spawn(entities, components)
    }
}

impl<E, C, R> World<E, C, R>
where
    C: Components,
{
    /// Checks if the component bundle was previously registered in the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn is_registered<B>(&self) -> bool
    where
        B: Bundle,
    {
        let Self { components, .. } = self;
        B::Storages::is_registered(components)
    }
}

impl<E, C, R> World<E, C, R>
where
    C: WithComponents,
{
    /// Inserts storages of provided component bundle into the world,
    /// resulting in a world with a new type of the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn with_components<B>(
        self,
        bundle: B::Storages,
    ) -> World<E, <B::Storages as StorageBundle>::With<C>, R>
    where
        B: Bundle,
    {
        let Self {
            entities,
            components,
            resources,
        } = self;

        let components = B::Storages::with(components, bundle);
        World::with(entities, components, resources)
    }
}

impl<E, C, R> World<E, C, R>
where
    C: ComponentsMut,
{
    /// Registers the component bundle in the current world with provided storage bundle.
    /// Returns previous value of the storage bundle, or [`None`] if the component bundle was not registered.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn register<B>(&mut self, bundle: B::Storages) -> Option<B::Storages>
    where
        B: Bundle,
    {
        let Self { components, .. } = self;
        B::Storages::register(components, bundle)
    }

    /// Unregisters the component bundle from the current world and returns component storage bundle.
    /// Returns [`None`] if the component bundle was not registered.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn unregister<B>(&mut self) -> Option<B::Storages>
    where
        B: Bundle,
    {
        let Self { components, .. } = self;
        B::Storages::unregister(components)
    }
}

impl<E, C, R> World<E, C, R>
where
    C: TryComponentsMut,
{
    /// Tries to register the component bundle in the current world with provided component storage bundle.
    /// Returns previous value of the storage bundle, or [`None`] if the component bundle was not registered.
    ///
    /// # Errors
    ///
    /// This function will return an error if the world will fail to register provided component bundle.
    /// Conditions of failure are provided by implementation of the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`register`][World::register()] method.
    pub fn try_register<B>(&mut self, bundle: B::Storages) -> Result<Option<B::Storages>, C::Err>
    where
        B: Bundle,
        B::Storages: StorageTryBundle,
    {
        let Self { components, .. } = self;
        B::Storages::try_register(components, bundle)
    }
}

impl<E, C, R> World<E, C, R>
where
    R: Resources,
{
    /// Checks if the resource bundle was previously inserted in the current world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn contains_res<B>(&self) -> bool
    where
        B: ResourceBundle,
    {
        let Self { resources, .. } = self;
        B::contains(resources)
    }

    /// Retrieves a reference to the inserted resource bundle.
    /// Returns [`None`] if the resource bundle was not inserted in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get_res<B>(&self) -> Option<B::Ref<'_>>
    where
        B: ResourceGetBundle,
    {
        let Self { resources, .. } = self;
        B::get(resources)
    }

    /// Retrieves a mutable reference to the inserted resource bundle.
    /// Returns [`None`] if the resource bundle was not inserted in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get_res_mut<B>(&mut self) -> Option<B::RefMut<'_>>
    where
        B: ResourceGetBundleMut,
    {
        let Self { resources, .. } = self;
        B::get_mut(resources)
    }

    /// Retrieves a reference to the inserted resource bundle.
    ///
    /// Unlike other methods, this guarantees that all the components
    /// of provided bundle always exist in resource registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provide_res<B, I>(&self) -> B::Ref<'_>
    where
        B: ResourceProvideBundle<R, I>,
    {
        let Self { resources, .. } = self;
        B::provide(resources)
    }

    /// Retrieves a mutable reference to the inserted resource bundle.
    ///
    /// Unlike other methods, this guarantees that all the components
    /// of provided bundle always exist in resource registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provide_res_mut<B, I>(&mut self) -> B::RefMut<'_>
    where
        B: ResourceProvideBundleMut<R, I>,
    {
        let Self { resources, .. } = self;
        B::provide_mut(resources)
    }
}

impl<E, C, R> World<E, C, R>
where
    R: WithResources,
{
    /// Inserts provided resource bundle into the world,
    /// resulting in a world with a new type of the resource registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn with_res<B>(self, bundle: B) -> World<E, C, B::With<R>>
    where
        B: ResourceBundle,
    {
        let Self {
            entities,
            components,
            resources,
        } = self;

        let resources = B::with(resources, bundle);
        World::with(entities, components, resources)
    }
}

impl<E, C, R> World<E, C, R>
where
    R: ResourcesMut,
{
    /// Insert provided resource bundle into the current world.
    /// Returns previous value of the resource bundle, or [`None`] if the resource bundle was not in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn insert_res<B>(&mut self, bundle: B) -> Option<B>
    where
        B: ResourceBundle,
    {
        let Self { resources, .. } = self;
        B::insert(resources, bundle)
    }

    /// Removes the resource bundle from the current world and returns value of removed resource bundle.
    /// Returns [`None`] if the resource bundle was not in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn remove_res<B>(&mut self) -> Option<B>
    where
        B: ResourceBundle,
    {
        let Self { resources, .. } = self;
        B::remove(resources)
    }
}

impl<E, C, R> World<E, C, R>
where
    R: TryResourcesMut,
{
    /// Tries to insert provided resource bundle into the current world.
    /// Returns previous value of the resource bundle, or [`None`] if the resource bundle was not in the world.
    ///
    /// # Errors
    ///
    /// This function will return an error if the world will fail to insert provided resource bundle.
    /// Conditions of failure are provided by implementation of the resource registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`insert_res`][World::insert_res()] method.
    pub fn try_insert_res<B>(&mut self, bundle: B) -> Result<Option<B>, R::Err>
    where
        B: ResourceTryBundle,
    {
        let Self { resources, .. } = self;
        B::try_insert(resources, bundle)
    }
}

impl<E, C, R> World<E, C, R>
where
    E: Entities,
    C: Components,
{
    /// Creates new [entity builder](EntityBuilder) from provided initial value,
    /// which allows to create new entity *lazily*.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn builder_from<T>(&mut self, init: T) -> EntityBuilder<'_, E, C, T> {
        let Self {
            entities,
            components,
            ..
        } = self;
        EntityBuilder::new(entities, components, init)
    }

    /// Creates new entity with provided bundle in the current world.
    ///
    /// All components of the bundle will be attached to the newly created entity.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of components in the bundle
    /// was not registered in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn create_with<B>(&mut self, bundle: B) -> Result<E::Entity, NotRegisteredError>
    where
        B: Bundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        self.builder_from(bundle).build()
    }

    /// Tries to create new entity with provided bundle in the current world.
    ///
    /// All components of the bundle will be attached to the newly created entity.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if one of components in the bundle was not registered in the world
    /// or the world will fail to attach some component of the bundle to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`create_with`][World::create_with()] method.
    pub fn try_bundle_create_with<B>(
        &mut self,
        bundle: B,
    ) -> Result<E::Entity, TryBundleError<B::Err>>
    where
        B: TryBundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        self.builder_from(bundle).try_bundle_build()
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
    pub fn attach<B>(
        &mut self,
        entity: E::Entity,
        bundle: B,
    ) -> Result<Option<B>, EntityError<E::Entity>>
    where
        B: Bundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::attach(components, entity, bundle)?;
        Ok(bundle)
    }

    /// Tries to attach provided bundle to the entity in the world.
    ///
    /// Returns previous bundle data attached to the entity earlier.
    /// Returns [`None`] if there was no bundle attached to the entity or some of bundle components are missing.
    ///
    /// # Errors
    ///
    /// This function will return an error if if provided entity does not present in the world,
    /// one of bundle components was not registered in the world
    /// or storage of some component of bundle will fail to attach it to the entity.
    /// Conditions of failure are provided by implementation of the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`attach`][World::attach()] method.
    pub fn try_attach<B>(
        &mut self,
        entity: E::Entity,
        bundle: B,
    ) -> Result<Option<B>, TryAttachError<B::Err, E::Entity>>
    where
        B: TryBundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::try_attach(components, entity, bundle)?;
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
    pub fn is_attached<B>(&self, entity: E::Entity) -> Result<bool, EntityError<E::Entity>>
    where
        B: Bundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let is_attached = B::is_attached(components, entity)?;
        Ok(is_attached)
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
    pub fn remove<B>(&mut self, entity: E::Entity) -> Result<Option<B>, EntityError<E::Entity>>
    where
        B: Bundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::remove(components, entity)?;
        Ok(bundle)
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
    pub fn get<B>(&self, entity: E::Entity) -> Result<Option<B::Ref<'_>>, EntityError<E::Entity>>
    where
        B: GetBundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::get(components, entity)?;
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
    pub fn get_mut<B>(
        &mut self,
        entity: E::Entity,
    ) -> Result<Option<B::RefMut<'_>>, EntityError<E::Entity>>
    where
        B: GetBundleMut,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error.into());
        }
        let bundle = B::get_mut(components, entity)?;
        Ok(bundle)
    }

    /// Retrieves a reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have any of bundle components.
    ///
    /// Unlike other methods, this guarantees that all the components
    /// of provided bundle always exist in component registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provide<B, I>(
        &self,
        entity: E::Entity,
    ) -> Result<Option<B::Ref<'_>>, NotPresentError<E::Entity>>
    where
        B: ProvideBundle<C, I>,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let bundle = B::provide(components, entity);
        Ok(bundle)
    }

    /// Retrieves a mutable reference to the bundle which components are attached to provided entity.
    /// Returns [`None`] if provided entity does not have any of bundle components.
    ///
    /// Unlike other methods, this guarantees that all the components
    /// of provided bundle always exist in component registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provide_mut<B, I>(
        &mut self,
        entity: E::Entity,
    ) -> Result<Option<B::RefMut<'_>>, NotPresentError<E::Entity>>
    where
        B: ProvideBundleMut<C, I>,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let bundle = B::provide_mut(components, entity);
        Ok(bundle)
    }

    /// Creates new view by provided readonly query.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn view<Q>(&self) -> Option<View<'_, Q, E>>
    where
        Q: ReadonlyQuery<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;
        View::new_readonly(entities, components)
    }

    /// Creates new mutable view by provided query.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn view_mut<Q>(&mut self) -> Option<View<'_, Q, E>>
    where
        Q: Query<Entity = E::Entity>,
    {
        let Self {
            entities,
            components,
            ..
        } = self;
        View::new(entities, components)
    }
}

impl<E, C, R> World<E, C, R>
where
    E: TryEntities,
    C: Components,
{
    /// Tries to create new entity with provided bundle in the current world.
    ///
    /// All components of the bundle will be attached to the newly created entity.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if one of components in the bundle was not registered in the world
    /// or the world will fail to create new entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`create_with`][World::create_with()] method.
    pub fn try_entity_create_with<B>(
        &mut self,
        bundle: B,
    ) -> Result<E::Entity, TryEntityBuildError<E::Err>>
    where
        B: Bundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        self.builder_from(bundle).try_entity_build()
    }

    /// Tries to create new entity with provided bundle in the current world.
    ///
    /// All components of the bundle will be attached to the newly created entity.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if one of components in the bundle was not registered in the world,
    /// the world will fail to create new entity
    /// or the world will fail to attach some component of the bundle to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// This is the fallible version of [`create_with`][World::create_with()] method.
    pub fn try_create_with<B>(
        &mut self,
        bundle: B,
    ) -> Result<E::Entity, TryBuildError<E::Err, B::Err>>
    where
        B: TryBundle,
        B::Storages: StorageBundle<Entity = E::Entity>,
    {
        self.builder_from(bundle).try_build()
    }
}

impl<E, C, R> World<E, C, R>
where
    E: Entities,
    C: ComponentsMut,
    R: ResourcesMut,
{
    /// Checks if the world contains no entities, components or resources.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn is_empty(&self) -> bool {
        let Self {
            entities,
            components,
            resources,
        } = self;
        entities.is_empty() && components.is_empty() && resources.is_empty()
    }

    /// Clears the current world, destroying all entities, their components and all resources of the world.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn clear(&mut self) {
        let Self {
            entities,
            components,
            resources,
        } = self;

        entities.clear();
        components.clear();
        resources.clear();
    }
}
