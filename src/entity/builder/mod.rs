//! Provides a builder pattern implementation for entities.

use hlist2::{ops::Append, Cons, Nil};

use crate::component::{
    bundle::Bundle, error::NotRegisteredResult, registry::Registry as Components,
};

use super::{registry::Registry as Entities, Entity};

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
    pub fn build<E, C>(self, entities: &mut E, components: &mut C) -> NotRegisteredResult<Entity>
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
}

impl Default for EntityBuilder {
    fn default() -> Self {
        Self::empty()
    }
}

/// Type of [entity builder][builder] with state
/// provided by entity and component registries.
///
/// This struct uses [entity builder][builder] stateless implementation to build new entity.
///
/// [builder]: self::EntityBuilder
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
    pub fn build(self) -> NotRegisteredResult<Entity> {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.build(entities, components)
    }
}

/// Converts stateful entity builder into stateless.
impl<'state, E, C, T> From<StateEntityBuilder<'state, E, C, T>> for EntityBuilder<T> {
    fn from(builder: StateEntityBuilder<'state, E, C, T>) -> Self {
        let StateEntityBuilder {
            entities: _,
            components: _,
            builder,
        } = builder;
        builder
    }
}

#[doc(hidden)]
pub trait Bundles: Append {
    fn check_all<C>(components: &mut C) -> NotRegisteredResult<()>
    where
        C: Components;

    fn attach_all<C>(self, components: &mut C, entity: Entity) -> NotRegisteredResult<()>
    where
        C: Components;
}

impl Bundles for Nil {
    fn check_all<C>(_: &mut C) -> NotRegisteredResult<()>
    where
        C: Components,
    {
        Ok(())
    }

    fn attach_all<C>(self, _: &mut C, _: Entity) -> NotRegisteredResult<()>
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
    fn check_all<C>(components: &mut C) -> NotRegisteredResult<()>
    where
        C: Components,
    {
        Head::is_attached(components, Entity::null())?;
        Tail::check_all(components)
    }

    fn attach_all<C>(self, components: &mut C, entity: Entity) -> NotRegisteredResult<()>
    where
        C: Components,
    {
        let Cons(head, tail) = self;
        Head::attach(components, entity, head)?;
        tail.attach_all(components, entity)
    }
}
