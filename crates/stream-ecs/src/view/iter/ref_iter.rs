use crate::{
    entity::Entity,
    view::query::{AsReadonly, Query},
};

/// Iterator for the borrow of the view.
pub struct ViewRefIter<'fetch, Q, E>
where
    Q: AsReadonly,
    E: Iterator<Item = Entity>,
{
    entities: E,
    fetch: Q::ReadonlyRef<'fetch>,
}

impl<'fetch, Q, E> ViewRefIter<'fetch, Q, E>
where
    Q: AsReadonly,
    E: Iterator<Item = Entity>,
{
    pub(in crate::view) fn new<I>(entities: I, fetch: Q::ReadonlyRef<'fetch>) -> Self
    where
        I: IntoIterator<IntoIter = E>,
    {
        let entities = entities.into_iter();
        Self { entities, fetch }
    }
}

impl<'fetch, Q, E> Iterator for ViewRefIter<'fetch, Q, E>
where
    Q: AsReadonly,
    E: Iterator<Item = Entity>,
{
    type Item = <Q::Readonly as Query>::Item<'fetch>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            ref mut entities,
            fetch,
        } = *self;
        let item = loop {
            let entity = entities.next()?;
            let item = Q::readonly_ref_fetch(fetch, entity);
            if let Some(item) = item {
                break item;
            }
        };
        Some(item)
    }
}
