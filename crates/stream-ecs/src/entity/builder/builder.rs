#![allow(clippy::module_inception)]

use hlist::{ops::Append, Nil};

use crate::{
    component::{
        bundle::{Bundle, NotRegisteredError, TryBundle, TryBundleError},
        registry::Registry as Components,
    },
    entity::{
        builder::{TryBuildError, TryEntityBuildError},
        registry::{Registry as Entities, TryRegistry as TryEntities},
        Entity,
    },
};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityBuilder<T = Nil>(T);

impl EntityBuilder {
    /// Creates an empty entity builder.
    ///
    /// Returns new builder without any components attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn empty() -> Self {
        EntityBuilder(Nil)
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
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
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
    T: Bundle,
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
        let Self(bundle) = self;

        let entity = entities.create();
        if let Err(err) = T::attach(components, entity, bundle) {
            let _ = entities.destroy(entity);
            return Err(err);
        }
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
        let Self(bundle) = self;

        let entity = match entities.try_create() {
            Ok(entity) => entity,
            Err(err) => return Err(TryEntityBuildError::Entities(err)),
        };
        if let Err(err) = T::attach(components, entity, bundle) {
            let _ = entities.destroy(entity);
            return Err(err.into());
        }
        Ok(entity)
    }
}

impl<T> EntityBuilder<T>
where
    T: TryBundle,
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
        let Self(bundle) = self;

        let entity = entities.create();
        if let Err(err) = T::try_attach(components, entity, bundle) {
            let _ = entities.destroy(entity);
            return Err(err);
        }
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
        let Self(bundle) = self;

        let entity = entities.create();
        if let Err(err) = T::try_attach(components, entity, bundle) {
            let _ = entities.destroy(entity);
            return Err(err.into());
        }
        Ok(entity)
    }
}

impl Default for EntityBuilder {
    fn default() -> Self {
        Self::empty()
    }
}
