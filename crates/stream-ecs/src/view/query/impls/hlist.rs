use core::any::Any;

use hlist::{Cons, Nil};
use ref_kind::RefKind;

use crate::{
    component::registry::Registry as Components,
    dependency::{dependency_from_iter, Dependency},
    view::query::{AsReadonly, IntoReadonly, Query, ReadonlyQuery},
};

impl<Head> Query for Cons<Head, Nil>
where
    Head: Query,
{
    type Entity = Head::Entity;

    type Item<'item> = Cons<Head::Item<'item>, Nil>;

    type Fetch<'fetch> = Cons<Head::Fetch<'fetch>, Nil>;

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let head = Head::new_fetch(components)?;
        let fetch = Cons(head, Nil);
        Some(fetch)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'borrow>> {
        let Cons(head, _) = fetch;
        let head = Head::fetch(head, entity)?;
        let item = Cons(head, Nil);
        Some(item)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: Self::Entity) -> bool {
        let Cons(head, _) = fetch;
        Head::satisfies(head, entity)
    }
}

impl<Head, Tail> Query for Cons<Head, Tail>
where
    Head: Query,
    Tail: Query<Entity = Head::Entity>,
    for<'any> Head::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> Tail::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
{
    type Entity = Head::Entity;

    type Item<'item> = Cons<Head::Item<'item>, Tail::Item<'item>>;

    type Fetch<'fetch> = Cons<Head::Fetch<'fetch>, Tail::Fetch<'fetch>>;

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let iter = components
            .iter_mut()
            .map(|storage| Some(RefKind::from(storage.as_any_mut())));
        dependency_from_iter(iter).ok()
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'borrow>> {
        let Cons(head, tail) = fetch;
        let head = Head::fetch(head, entity)?;
        let tail = Tail::fetch(tail, entity)?;
        let item = Cons(head, tail);
        Some(item)
    }

    fn satisfies(fetch: &Self::Fetch<'_>, entity: Self::Entity) -> bool {
        let Cons(head, tail) = fetch;
        Head::satisfies(head, entity) && Tail::satisfies(tail, entity)
    }
}

impl<Head> IntoReadonly for Cons<Head, Nil>
where
    Head: IntoReadonly,
{
    type Readonly = Cons<Head::Readonly, Nil>;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        let Cons(head, nil) = fetch;
        let head = Head::into_readonly(head);
        Cons(head, nil)
    }
}

impl<Head, Tail> IntoReadonly for Cons<Head, Tail>
where
    Head: IntoReadonly,
    Tail: IntoReadonly<Entity = Head::Entity>,
    for<'any> Head::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> Tail::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> <Head::Readonly as Query>::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> <Tail::Readonly as Query>::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
{
    type Readonly = Cons<Head::Readonly, Tail::Readonly>;

    fn into_readonly(fetch: Self::Fetch<'_>) -> <Self::Readonly as Query>::Fetch<'_> {
        let Cons(head, tail) = fetch;
        let head = Head::into_readonly(head);
        let tail = Tail::into_readonly(tail);
        Cons(head, tail)
    }
}

impl<Head> AsReadonly for Cons<Head, Nil>
where
    Head: AsReadonly,
{
    type ReadonlyRef<'borrow> = Cons<Head::ReadonlyRef<'borrow>, Nil>;

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        let Cons(head, _) = fetch;
        let head = Head::as_readonly(head);
        Cons(head, Nil)
    }

    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        entity: Self::Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>> {
        let Cons(head, _) = fetch;
        let head = Head::readonly_ref_fetch(head, entity)?;
        let item = Cons(head, Nil);
        Some(item)
    }

    fn readonly_ref_satisfies(fetch: Self::ReadonlyRef<'_>, entity: Self::Entity) -> bool {
        let Cons(head, _) = fetch;
        Head::readonly_ref_satisfies(head, entity)
    }
}

impl<Head, Tail> AsReadonly for Cons<Head, Tail>
where
    Head: AsReadonly,
    Tail: AsReadonly<Entity = Head::Entity>,
    for<'any> Head::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> Tail::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> <Head::Readonly as Query>::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> <Tail::Readonly as Query>::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
{
    type ReadonlyRef<'borrow> = Cons<Head::ReadonlyRef<'borrow>, Tail::ReadonlyRef<'borrow>>;

    fn as_readonly<'borrow>(fetch: &'borrow Self::Fetch<'_>) -> Self::ReadonlyRef<'borrow> {
        let Cons(head, tail) = fetch;
        let head = Head::as_readonly(head);
        let tail = Tail::as_readonly(tail);
        Cons(head, tail)
    }

    fn readonly_ref_fetch(
        fetch: Self::ReadonlyRef<'_>,
        entity: Self::Entity,
    ) -> Option<<Self::Readonly as Query>::Item<'_>> {
        let Cons(head, tail) = fetch;
        let head = Head::readonly_ref_fetch(head, entity)?;
        let tail = Tail::readonly_ref_fetch(tail, entity)?;
        let item = Cons(head, tail);
        Some(item)
    }

    fn readonly_ref_satisfies(fetch: Self::ReadonlyRef<'_>, entity: Self::Entity) -> bool {
        let Cons(head, tail) = fetch;
        Head::readonly_ref_satisfies(head, entity) && Tail::readonly_ref_satisfies(tail, entity)
    }
}

impl<Head> ReadonlyQuery for Cons<Head, Nil>
where
    Head: ReadonlyQuery,
{
    fn new_readonly_fetch<C>(components: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let head = Head::new_readonly_fetch(components)?;
        let fetch = Cons(head, Nil);
        Some(fetch)
    }

    fn readonly_fetch<'fetch>(
        fetch: &Self::Fetch<'fetch>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'fetch>> {
        let Cons(head, _) = fetch;
        let head = Head::readonly_fetch(head, entity)?;
        let item = Cons(head, Nil);
        Some(item)
    }
}

impl<Head, Tail> ReadonlyQuery for Cons<Head, Tail>
where
    Head: ReadonlyQuery,
    Tail: ReadonlyQuery<Entity = Head::Entity>,
    for<'any> Head::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
    for<'any> Tail::Fetch<'any>: Dependency<Option<RefKind<'any, dyn Any>>>,
{
    fn new_readonly_fetch<C>(components: &C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let head = Head::new_readonly_fetch(components)?;
        let tail = Tail::new_readonly_fetch(components)?;
        let fetch = Cons(head, tail);
        Some(fetch)
    }

    fn readonly_fetch<'fetch>(
        fetch: &Self::Fetch<'fetch>,
        entity: Self::Entity,
    ) -> Option<Self::Item<'fetch>> {
        let Cons(head, tail) = fetch;
        let head = Head::readonly_fetch(head, entity)?;
        let tail = Tail::readonly_fetch(tail, entity)?;
        let item = Cons(head, tail);
        Some(item)
    }
}
