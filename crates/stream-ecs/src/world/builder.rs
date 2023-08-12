use hlist::{ops::Append, Nil};

use crate::{
    component::{
        bundle::{Bundle, NotRegisteredError, TryBundle, TryBundleError},
        registry::Registry as Components,
        storage::bundle::Bundle as StorageBundle,
    },
    entity::{
        builder::{self, TryBuildError, TryEntityBuildError},
        registry::{Registry as Entities, TryRegistry as TryEntities},
    },
};

/// Type of entity builder with state provided by entity and component registries.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[must_use = "Entity builder will not create new entity unless .build() was called"]
#[derive(Debug)]
pub struct EntityBuilder<'state, E, C, T = Nil> {
    entities: &'state mut E,
    components: &'state mut C,
    builder: builder::EntityBuilder<T>,
}

impl<'state, E, C> EntityBuilder<'state, E, C> {
    /// Creates an empty entity builder with provided entity and component registries.
    ///
    /// Returns new builder without any components attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new(entities: &'state mut E, components: &'state mut C) -> Self {
        let builder = builder::EntityBuilder::empty();
        Self::from_builder(entities, components, builder)
    }
}

impl<'state, E, C, T> EntityBuilder<'state, E, C, T> {
    /// Creates stateful entity builder from provided stateless builder
    /// and entity and component registries.
    ///
    /// Returns new builder with all the components from the stateless builder.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn from_builder(
        entities: &'state mut E,
        components: &'state mut C,
        builder: builder::EntityBuilder<T>,
    ) -> Self {
        Self {
            entities,
            components,
            builder,
        }
    }
}

impl<'state, E, C, T> EntityBuilder<'state, E, C, T>
where
    T: Append,
{
    /// Inserts new bundle to the builder.
    ///
    /// Contents of inserted bundles will be attached to the entity
    /// in the order of their insertion.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn with<B>(self, bundle: B) -> EntityBuilder<'state, E, C, T::Output<B>>
    where
        B: Bundle,
    {
        let Self {
            entities,
            components,
            builder,
        } = self;
        EntityBuilder {
            entities,
            components,
            builder: builder.with(bundle),
        }
    }
}

impl<'state, E, C, T> EntityBuilder<'state, E, C, T>
where
    T: Bundle,
{
    /// Creates new entity builder from provided component bundle
    /// and with provided entity and component registries.
    ///
    /// Returns new builder with all the components of the bundle.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn from_bundle(entities: &'state mut E, components: &'state mut C, bundle: T) -> Self {
        let builder = builder::EntityBuilder::from_bundle(bundle);
        Self::from_builder(entities, components, builder)
    }
}

impl<'state, E, C, T> EntityBuilder<'state, E, C, T>
where
    T: Bundle,
    T::Storages: StorageBundle<Entity = E::Entity>,
    E: Entities,
    C: Components,
{
    /// Builds new entity from the builder.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of components in provided bundles
    /// was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// Note that the contents of inserted bundles are attached to the newly created entity
    /// in the order of their insertion.
    pub fn build(self) -> Result<E::Entity, NotRegisteredError> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.build(entities, components)
    }
}

impl<'state, E, C, T> EntityBuilder<'state, E, C, T>
where
    T: Bundle,
    T::Storages: StorageBundle<Entity = E::Entity>,
    E: TryEntities,
    C: Components,
{
    /// Tries to build new entity from the builder.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of components in provided bundles
    /// was not registered in the component registry or the entity registry will fail to create new entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// Note that the contents of inserted bundles are attached to the newly created entity
    /// in the order of their insertion.
    ///
    /// This is the fallible version of [`build`][EntityBuilder::build()] method.
    pub fn try_entity_build(self) -> Result<E::Entity, TryEntityBuildError<E::Err>> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.try_entity_build(entities, components)
    }
}

impl<'state, E, C, T> EntityBuilder<'state, E, C, T>
where
    T: TryBundle,
    T::Storages: StorageBundle<Entity = E::Entity>,
    E: Entities,
    C: Components,
{
    /// Tries to build new entity from the builder.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of components in provided bundles
    /// was not registered in the component registry
    /// or storage of some component will fail to attach it to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// Note that the contents of inserted bundles are attached to the newly created entity
    /// in the order of their insertion.
    ///
    /// This is the fallible version of [`build`][EntityBuilder::build()] method.
    pub fn try_bundle_build(self) -> Result<E::Entity, TryBundleError<T::Err>> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.try_bundle_build(entities, components)
    }
}

impl<'state, E, C, T> EntityBuilder<'state, E, C, T>
where
    T: TryBundle,
    T::Storages: StorageBundle<Entity = E::Entity>,
    E: TryEntities,
    C: Components,
{
    /// Tries to build new entity from the builder.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of components in provided bundles
    /// was not registered in the component registry, the entity registry will fail to create new entity
    /// or storage of some component will fail to attach it to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    ///
    /// Note that the contents of inserted bundles are attached to the newly created entity
    /// in the order of their insertion.
    ///
    /// This is the fallible version of [`build`][EntityBuilder::build()] method.
    pub fn try_build(self) -> Result<E::Entity, TryBuildError<E::Err, T::Err>> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.try_build(entities, components)
    }
}

/// Converts stateful entity builder into stateless.
impl<'state, E, C, T> From<EntityBuilder<'state, E, C, T>> for builder::EntityBuilder<T> {
    fn from(builder: EntityBuilder<'state, E, C, T>) -> Self {
        let EntityBuilder { builder, .. } = builder;
        builder
    }
}
