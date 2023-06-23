use crate::{
    entity::{
        registry::{NotPresentError, Registry as Entities},
        Entity,
    },
    view::{
        self,
        iter::ViewRefIter,
        query::{AsReadonly, Query},
    },
};

/// Stateful borrow of the view.
pub struct ViewRef<'state, Q, E>
where
    Q: AsReadonly,
    E: Entities,
{
    entities: &'state E,
    view_ref: view::ViewRef<'state, Q>,
}

impl<'state, Q, E> ViewRef<'state, Q, E>
where
    Q: AsReadonly,
    E: Entities,
{
    pub(super) fn new(entities: &'state E, view_ref: view::ViewRef<'state, Q>) -> Self {
        Self { entities, view_ref }
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
    pub fn get(
        &self,
        entity: Entity,
    ) -> Result<Option<<Q::Readonly as Query>::Item<'state>>, NotPresentError> {
        let Self { entities, view_ref } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let item = view_ref.get(entity);
        Ok(item)
    }

    /// Turn this view into an iterator of entities and their data.
    pub fn iter(&self) -> ViewRefIter<'state, Q, E::Iter<'state>> {
        self.into_iter()
    }
}

impl<'state, Q, E> Clone for ViewRef<'state, Q, E>
where
    Q: AsReadonly,
    E: Entities,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'state, Q, E> Copy for ViewRef<'state, Q, E>
where
    Q: AsReadonly,
    E: Entities,
{
}

impl<'state, Q, E> IntoIterator for &ViewRef<'state, Q, E>
where
    Q: AsReadonly,
    E: Entities,
{
    type Item = <Q::Readonly as Query>::Item<'state>;

    type IntoIter = ViewRefIter<'state, Q, E::Iter<'state>>;

    fn into_iter(self) -> Self::IntoIter {
        let ViewRef { entities, view_ref } = self;
        let entities = entities.iter();
        view_ref.iter(entities)
    }
}
