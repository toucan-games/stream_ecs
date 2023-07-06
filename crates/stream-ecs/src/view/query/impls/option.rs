use crate::{
    component::registry::Registry as Components,
    entity::Entity,
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl<E, Q> Query<E> for Option<Q>
where
    E: Entity,
    Q: Query<E>,
{
    type Item<'item> = Option<Q::Item<'item>>;

    type Fetch<'fetch> = Option<Q::Fetch<'fetch>>;

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let fetch = Q::new_fetch(components);
        Some(fetch)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: E,
    ) -> Option<Self::Item<'borrow>> {
        let Some(fetch) = fetch else {
            return Some(None);
        };
        let item = Q::fetch(fetch, entity);
        Some(item)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: E) -> bool {
        let Some(fetch) = fetch else {
            return true;
        };
        Q::satisfies(fetch, entity)
    }
}

impl<E, Q> IntoReadonly<E> for Option<Q>
where
    E: Entity,
    Q: IntoReadonly<E>,
{
    type Readonly = Option<Q::Readonly>;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query<E>>::Fetch<'_> {
        fetch.map(Q::into_readonly)
    }
}

impl<E, Q> AsReadonly<E> for Option<Q>
where
    E: Entity,
    Q: AsReadonly<E>,
{
    type ReadonlyRef<'borrow> = Option<Q::ReadonlyRef<'borrow>>;

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        fetch.as_ref().map(Q::as_readonly)
    }

    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        entity: E,
    ) -> Option<<Self::Readonly as Query<E>>::Item<'_>> {
        let Some(fetch) = fetch else {
            return Some(None);
        };
        let item = Q::readonly_ref_fetch(fetch, entity);
        Some(item)
    }

    fn readonly_ref_satisfies(fetch: Self::ReadonlyRef<'_>, entity: E) -> bool {
        let Some(fetch) = fetch else {
            return true;
        };
        Q::readonly_ref_satisfies(fetch, entity)
    }
}

impl<E, Q> ReadonlyQuery<E> for Option<Q>
where
    E: Entity,
    Q: ReadonlyQuery<E>,
{
    fn new_readonly_fetch<C>(components: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let fetch = Q::new_readonly_fetch(components);
        Some(fetch)
    }

    fn readonly_fetch<'fetch>(
        fetch: &Self::Fetch<'fetch>,
        entity: E,
    ) -> Option<Self::Item<'fetch>> {
        let Some(fetch) = fetch else {
            return Some(None);
        };
        let item = Q::readonly_fetch(fetch, entity);
        Some(item)
    }
}
