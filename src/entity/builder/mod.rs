//! Provides a builder pattern implementation for entities.

use hlist2::{ops::Append, Cons, Nil};

use crate::component::{bundle::Bundle, registry::Registry as Components};

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
    pub fn into_stateful<'state, E, C>(
        self,
        entities: &'state mut E,
        components: &'state mut C,
    ) -> StatefulEntityBuilder<'state, E, C, T> {
        StatefulEntityBuilder {
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
    /// Contents of inserted bundles are attached to the newly created entity
    /// in the order of their insertion.
    pub fn build<E, C>(self, entities: &mut E, components: &mut C) -> Entity
    where
        E: Entities,
        C: Components,
    {
        let Self(bundles) = self;
        let entity = entities.create();
        bundles.attach_all(components, entity);
        entity
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
pub struct StatefulEntityBuilder<'state, E, C, T = Nil> {
    entities: &'state mut E,
    components: &'state mut C,
    builder: EntityBuilder<T>,
}

impl<'state, E, C> StatefulEntityBuilder<'state, E, C> {
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

impl<'state, E, C, T> StatefulEntityBuilder<'state, E, C, T>
where
    T: Append,
{
    /// Inserts new bundle to the builder.
    ///
    /// Contents of inserted bundles will be attached to the entity
    /// in the order of their insertion.
    pub fn with<B>(self, bundle: B) -> StatefulEntityBuilder<'state, E, C, T::Output<B>>
    where
        B: Bundle,
    {
        let Self {
            entities,
            components,
            builder,
        } = self;
        StatefulEntityBuilder {
            entities,
            components,
            builder: builder.with(bundle),
        }
    }
}

impl<'state, E, C, T> StatefulEntityBuilder<'state, E, C, T>
where
    T: Bundles,
    E: Entities,
    C: Components,
{
    /// Builds new entity from the builder.
    ///
    /// Contents of inserted bundles are attached to the newly created entity
    /// in the order of their insertion.
    pub fn build(self) -> Entity {
        let Self {
            entities,
            components,
            builder,
        } = self;
        builder.build(entities, components)
    }
}

/// Converts stateful entity builder into stateless.
impl<'state, E, C, T> From<StatefulEntityBuilder<'state, E, C, T>> for EntityBuilder<T> {
    fn from(builder: StatefulEntityBuilder<'state, E, C, T>) -> Self {
        let StatefulEntityBuilder {
            entities: _,
            components: _,
            builder,
        } = builder;
        builder
    }
}

#[doc(hidden)]
pub trait Bundles: Append {
    fn attach_all<C>(self, components: &mut C, entity: Entity)
    where
        C: Components;
}

impl Bundles for Nil {
    fn attach_all<C>(self, _: &mut C, _: Entity)
    where
        C: Components,
    {
    }
}

impl<Head, Tail> Bundles for Cons<Head, Tail>
where
    Head: Bundle,
    Tail: Bundles,
{
    fn attach_all<C>(self, components: &mut C, entity: Entity)
    where
        C: Components,
    {
        let Cons(head, tail) = self;
        Head::attach(components, entity, head);
        tail.attach_all(components, entity);
    }
}
