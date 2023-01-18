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
pub struct EntityBuilder<T = Nil>(T)
where
    T: Bundles;

impl EntityBuilder {
    /// Creates an empty entity builder.
    ///
    /// Returns new builder without any components attached to it.
    pub const fn empty() -> Self {
        EntityBuilder(Nil)
    }
}

impl<T> EntityBuilder<T>
where
    T: Bundles,
{
    /// Inserts new bundle to the builder.
    ///
    /// Contents of inserted bundles will be attached to the entity
    /// in the order of their insertion.
    pub fn with<B>(self, bundle: B) -> EntityBuilder<T::Output<B>>
    where
        B: Bundle,
        T::Output<B>: Bundles,
    {
        let Self(bundles) = self;
        let bundles = bundles.append(bundle);
        EntityBuilder(bundles)
    }

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
