#![allow(clippy::module_inception)]

use crate::{
    component::registry::Registry as Components,
    entity::{
        registry::{NotPresentError, Registry as Entities},
        Entity,
    },
    view::{
        self,
        iter::{ViewIter, ViewIterMut},
        query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
    },
};

use super::view_ref::ViewRef;

/// Stateful view of entities and their components.
pub struct View<'state, Q, E>
where
    Q: Query,
    E: Entities,
{
    entities: &'state E,
    view: view::View<'state, Q>,
}

impl<'state, Q, E> View<'state, Q, E>
where
    Q: Query,
    E: Entities,
{
    /// Creates new view of entities from provided entity and mutable component registries.
    pub fn new<C>(entities: &'state E, components: &'state mut C) -> Option<Self>
    where
        C: Components,
    {
        let view = view::View::new(components)?;
        Some(Self::from_view(entities, view))
    }

    /// Creates new view from provided entity registry and fetcher object.
    pub fn from_fetch(entities: &'state E, fetch: Q::Fetch<'state>) -> Self {
        let view = view::View::from_fetch(fetch);
        Self::from_view(entities, view)
    }

    /// Creates new stateful view from provided entity registry and view.
    pub fn from_view(entities: &'state E, view: view::View<'state, Q>) -> Self {
        Self { entities, view }
    }

    /// Get mutable items of the query by provided entity.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get_mut(&mut self, entity: Entity) -> Result<Option<Q::Item<'_>>, NotPresentError> {
        let Self { entities, view } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let item = view.get_mut(entity);
        Ok(item)
    }

    /// Turn this view into a mutable iterator of entities and their data.
    pub fn iter_mut(&mut self) -> ViewIterMut<'_, 'state, Q, E::Iter<'_>> {
        let Self { entities, view } = self;
        let entities = entities.iter();
        view.iter_mut(entities)
    }
}

impl<'state, Q, E> View<'state, Q, E>
where
    Q: IntoReadonly,
    E: Entities,
{
    /// Converts this view into readonly view.
    pub fn into_readonly(self) -> View<'state, Q::Readonly, E> {
        let Self { entities, view } = self;
        let view = view.into_readonly();
        View::from_view(entities, view)
    }
}

impl<'state, Q, E> View<'state, Q, E>
where
    Q: AsReadonly,
    E: Entities,
{
    /// Returns a borrow of the view.
    pub fn as_readonly(&self) -> ViewRef<'_, Q, E> {
        let Self { entities, view } = self;
        let view_ref = view.as_readonly();
        ViewRef::new(entities, view_ref)
    }
}

impl<'state, Q, E> View<'state, Q, E>
where
    Q: ReadonlyQuery,
    E: Entities,
{
    /// Creates new view of entities from provided entity and component registries.
    pub fn new_readonly<C>(entities: &'state E, components: &'state C) -> Option<Self>
    where
        C: Components,
    {
        let view = view::View::new_readonly(components)?;
        Some(Self::from_view(entities, view))
    }

    /// Get items of the query by provided entity.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity does not present in the entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get(&self, entity: Entity) -> Result<Option<Q::Item<'state>>, NotPresentError> {
        let Self { entities, view } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let item = view.get(entity);
        Ok(item)
    }

    /// Turn this view into an iterator of entities and their data.
    pub fn iter(&self) -> ViewIter<'_, 'state, Q, E::Iter<'_>> {
        self.into_iter()
    }
}

impl<'me, 'state, Q, E> IntoIterator for &'me View<'state, Q, E>
where
    Q: ReadonlyQuery,
    E: Entities,
{
    type Item = Q::Item<'state>;

    type IntoIter = ViewIter<'me, 'state, Q, E::Iter<'me>>;

    fn into_iter(self) -> Self::IntoIter {
        let View { entities, view } = self;
        let entities = entities.iter();
        view.iter(entities)
    }
}
