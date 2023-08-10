use crate::{
    entity::registry::{NotPresentError, Registry as Entities},
    view::{
        self,
        iter::ViewRefIter,
        query::{AsReadonly, IntoReadonly, Query},
    },
};

/// Stateful readonly borrow of the view.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub struct ViewRef<'state, Q, E>
where
    Q: AsReadonly<Entity = E::Entity>,
    E: Entities,
{
    entities: &'state E,
    view_ref: view::ViewRef<'state, Q>,
}

impl<'state, Q, E> ViewRef<'state, Q, E>
where
    Q: AsReadonly<Entity = E::Entity>,
    E: Entities,
{
    pub(super) fn new(entities: &'state E, view_ref: view::ViewRef<'state, Q>) -> Self {
        Self { entities, view_ref }
    }

    /// Checks if provided entity satisfies this query.
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
    pub fn satisfies(&self, entity: E::Entity) -> Result<bool, NotPresentError<E::Entity>> {
        let Self { entities, view_ref } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        Ok(view_ref.satisfies(entity))
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
    pub fn get(&self, entity: E::Entity) -> GetResult<'state, Q, E::Entity> {
        let Self { entities, view_ref } = self;

        if !entities.contains(entity) {
            let error = NotPresentError::new(entity);
            return Err(error);
        }
        let item = view_ref.get(entity);
        Ok(item)
    }

    /// Turn this view into an iterator of entities and their data.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn iter(&self) -> ViewRefIter<'state, Q, E::Iter<'state>> {
        self.into_iter()
    }
}

type GetResult<'a, Q, E> = Result<Option<ReadonlyItem<'a, Q>>, NotPresentError<E>>;

type ReadonlyItem<'a, Q> = <<Q as IntoReadonly>::Readonly as Query>::Item<'a>;

impl<'state, Q, E> Clone for ViewRef<'state, Q, E>
where
    Q: AsReadonly<Entity = E::Entity>,
    E: Entities,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'state, Q, E> Copy for ViewRef<'state, Q, E>
where
    Q: AsReadonly<Entity = E::Entity>,
    E: Entities,
{
}

impl<'state, Q, E> IntoIterator for &ViewRef<'state, Q, E>
where
    Q: AsReadonly<Entity = E::Entity>,
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
