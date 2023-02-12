//! Provides a builder pattern implementation for entities.

use either::Either;
use hlist::{ops::Append, Cons, Nil};

pub use self::error::{TryBuildError, TryEntityBuildError};

use crate::component::{
    bundle::{Bundle, NotRegisteredError, TryBundle, TryBundleError},
    registry::Registry as Components,
};

use super::{
    registry::{Registry as Entities, TryRegistry as TryEntities},
    Entity,
};

mod error;

/// Entity builder which creates new entity with provided components.
///
/// This struct could be used to create new entity *lazily*
/// based on some conditions which can change at runtime.
///
/// Note that the builder is *lazy* and does nothing unless being built.
/// Entity will be actually created on [`build`][build] function call.
///
/// [build]: EntityBuilder::build()
#[must_use = "Entity builder will not create new entity unless .build() was called"]
#[derive(Debug, Clone, Copy)]
pub struct EntityBuilder<T = Nil>(T);

impl EntityBuilder {
    /// Creates an empty entity builder.
    ///
    /// Returns new builder without any components attached to it.
    pub const fn empty() -> Self {
        EntityBuilder(Nil)
    }
}

impl<T> EntityBuilder<T> {
    /// Converts stateless entity builder into stateful.
    pub fn into_state_builder<'state, E, C>(
        self,
        entities: &'state mut E,
        components: &'state mut C,
    ) -> StateEntityBuilder<'state, E, C, T> {
        StateEntityBuilder {
            entities,
            components,
            builder: self,
        }
    }
}

impl<T> EntityBuilder<T>
where
    T: Append,
{
    /// Inserts new bundle to the builder.
    ///
    /// Contents of inserted bundles will be attached to the entity
    /// in the order of their insertion.
    pub fn with<B>(self, bundle: B) -> EntityBuilder<T::Output<B>>
    where
        B: Bundle,
    {
        let Self(bundles) = self;
        let bundles = bundles.append(bundle);
        EntityBuilder(bundles)
    }
}

impl<T> EntityBuilder<T>
where
    T: Bundles,
{
    /// Builds new entity from the builder.
    ///
    /// New entity will be created by provided entity registry, while components
    /// will be attached to the newly created entity with provided component registry.
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
    pub fn build<E, C>(
        self,
        entities: &mut E,
        components: &mut C,
    ) -> Result<Entity, NotRegisteredError>
    where
        E: Entities,
        C: Components,
    {
        let Self(bundles) = self;
        T::check_all(components)?;

        let entity = entities.create();
        bundles.attach_all(components, entity)?;
        Ok(entity)
    }

    /// Tries to build new entity from the builder.
    ///
    /// New entity will be created by provided entity registry, while components
    /// will be attached to the newly created entity with provided component registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of components in provided bundles
    /// was not registered in the component registry or provided entity registry will fail to create new entity.
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
    pub fn try_entity_build<E, C>(
        self,
        entities: &mut E,
        components: &mut C,
    ) -> Result<Entity, TryEntityBuildError<E::Err>>
    where
        E: TryEntities,
        C: Components,
    {
        let Self(bundles) = self;
        T::check_all(components)?;

        let entity = match entities.try_create() {
            Ok(entity) => entity,
            Err(err) => return Err(TryEntityBuildError::Entities(err)),
        };
        bundles.attach_all(components, entity)?;
        Ok(entity)
    }
}

impl<T> EntityBuilder<T>
where
    T: TryBundles,
{
    /// Tries to build new entity from the builder.
    ///
    /// New entity will be created by provided entity registry, while components
    /// will be attached to the newly created entity with provided component registry.
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
    pub fn try_bundle_build<E, C>(
        self,
        entities: &mut E,
        components: &mut C,
    ) -> Result<Entity, TryBundleError<T::Err>>
    where
        E: Entities,
        C: Components,
    {
        let Self(bundles) = self;
        T::check_all(components)?;

        let entity = entities.create();
        bundles.try_attach_all(components, entity)?;
        Ok(entity)
    }

    /// Tries to build new entity from the builder.
    ///
    /// New entity will be created by provided entity registry, while components
    /// will be attached to the newly created entity with provided component registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if one of components in provided bundles
    /// was not registered in the component registry,
    /// provided entity registry will fail to create new entity
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
    pub fn try_build<E, C>(
        self,
        entities: &mut E,
        components: &mut C,
    ) -> Result<Entity, TryBuildError<E::Err, T::Err>>
    where
        E: TryEntities,
        C: Components,
    {
        let Self(bundles) = self;
        T::check_all(components)?;

        let entity = match entities.try_create() {
            Ok(entity) => entity,
            Err(err) => return Err(TryBuildError::Entities(err)),
        };
        bundles.try_attach_all(components, entity)?;
        Ok(entity)
    }
}

impl Default for EntityBuilder {
    fn default() -> Self {
        Self::empty()
    }
}

/// Type of entity builder with state provided by entity and component registries.
///
/// This struct uses [entity builder](self::EntityBuilder) stateless implementation to build new entity.
#[must_use = "Entity builder will not create new entity unless .build() was called"]
#[derive(Debug)]
pub struct StateEntityBuilder<'state, E, C, T = Nil> {
    entities: &'state mut E,
    components: &'state mut C,
    builder: EntityBuilder<T>,
}

impl<'state, E, C> StateEntityBuilder<'state, E, C> {
    /// Creates an empty entity builder with provided entity and component registries.
    ///
    /// Returns new builder without any components attached to it.
    pub fn new(entities: &'state mut E, components: &'state mut C) -> Self {
        let builder = EntityBuilder::empty();
        Self {
            entities,
            components,
            builder,
        }
    }
}

impl<'state, E, C, T> StateEntityBuilder<'state, E, C, T>
where
    T: Append,
{
    /// Inserts new bundle to the builder.
    ///
    /// Contents of inserted bundles will be attached to the entity
    /// in the order of their insertion.
    pub fn with<B>(self, bundle: B) -> StateEntityBuilder<'state, E, C, T::Output<B>>
    where
        B: Bundle,
    {
        let Self {
            entities,
            components,
            builder,
        } = self;
        StateEntityBuilder {
            entities,
            components,
            builder: builder.with(bundle),
        }
    }
}

impl<'state, E, C, T> StateEntityBuilder<'state, E, C, T>
where
    T: Bundles,
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
    pub fn build(self) -> Result<Entity, NotRegisteredError> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.build(entities, components)
    }
}

impl<'state, E, C, T> StateEntityBuilder<'state, E, C, T>
where
    T: Bundles,
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
    /// This is the fallible version of [`build`][StateEntityBuilder::build()] method.
    pub fn try_entity_build(self) -> Result<Entity, TryEntityBuildError<E::Err>> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.try_entity_build(entities, components)
    }
}

impl<'state, E, C, T> StateEntityBuilder<'state, E, C, T>
where
    T: TryBundles,
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
    /// This is the fallible version of [`build`][StateEntityBuilder::build()] method.
    pub fn try_bundle_build(self) -> Result<Entity, TryBundleError<T::Err>> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.try_bundle_build(entities, components)
    }
}

impl<'state, E, C, T> StateEntityBuilder<'state, E, C, T>
where
    T: TryBundles,
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
    /// This is the fallible version of [`build`][StateEntityBuilder::build()] method.
    pub fn try_build(self) -> Result<Entity, TryBuildError<E::Err, T::Err>> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.try_build(entities, components)
    }
}

/// Converts stateful entity builder into stateless.
impl<'state, E, C, T> From<StateEntityBuilder<'state, E, C, T>> for EntityBuilder<T> {
    fn from(builder: StateEntityBuilder<'state, E, C, T>) -> Self {
        let StateEntityBuilder { builder, .. } = builder;
        builder
    }
}

#[doc(hidden)]
pub trait Bundles: Append {
    fn check_all<C>(components: &mut C) -> Result<(), NotRegisteredError>
    where
        C: Components;

    fn attach_all<C>(self, components: &mut C, entity: Entity) -> Result<(), NotRegisteredError>
    where
        C: Components;
}

impl Bundles for Nil {
    fn check_all<C>(_: &mut C) -> Result<(), NotRegisteredError>
    where
        C: Components,
    {
        Ok(())
    }

    fn attach_all<C>(self, _: &mut C, _: Entity) -> Result<(), NotRegisteredError>
    where
        C: Components,
    {
        Ok(())
    }
}

impl<Head, Tail> Bundles for Cons<Head, Tail>
where
    Head: Bundle,
    Tail: Bundles,
{
    fn check_all<C>(components: &mut C) -> Result<(), NotRegisteredError>
    where
        C: Components,
    {
        Head::is_attached(components, Entity::null())?;
        Tail::check_all(components)
    }

    fn attach_all<C>(self, components: &mut C, entity: Entity) -> Result<(), NotRegisteredError>
    where
        C: Components,
    {
        let Cons(head, tail) = self;
        Head::attach(components, entity, head)?;
        tail.attach_all(components, entity)
    }
}

#[doc(hidden)]
pub trait TryBundles: Bundles {
    type Err;

    fn try_attach_all<C>(
        self,
        components: &mut C,
        entity: Entity,
    ) -> Result<(), TryBundleError<Self::Err>>
    where
        C: Components;
}

impl TryBundles for Nil {
    type Err = core::convert::Infallible;

    fn try_attach_all<C>(self, _: &mut C, _: Entity) -> Result<(), TryBundleError<Self::Err>>
    where
        C: Components,
    {
        Ok(())
    }
}

impl<Head, Tail> TryBundles for Cons<Head, Tail>
where
    Head: TryBundle,
    Tail: TryBundles,
{
    type Err = Either<Head::Err, Tail::Err>;

    fn try_attach_all<C>(
        self,
        components: &mut C,
        entity: Entity,
    ) -> Result<(), TryBundleError<Self::Err>>
    where
        C: Components,
    {
        let Cons(head, tail) = self;
        match Head::try_attach(components, entity, head) {
            Ok(_) => (),
            Err(error) => match error {
                TryBundleError::NotRegistered(error) => return Err(error.into()),
                TryBundleError::Storage(error) => {
                    let error = Either::Left(error);
                    return Err(TryBundleError::Storage(error));
                }
            },
        }
        match tail.try_attach_all(components, entity) {
            Ok(_) => (),
            Err(error) => match error {
                TryBundleError::NotRegistered(error) => return Err(error.into()),
                TryBundleError::Storage(error) => {
                    let error = Either::Right(error);
                    return Err(TryBundleError::Storage(error));
                }
            },
        }
        Ok(())
    }
}
