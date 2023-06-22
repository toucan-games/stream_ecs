use core::any::Any;

use as_any::AsAny;
use hlist::{Cons, Nil};

use crate::{
    component::registry::Registry as Components,
    dependency::{dependency_from_iter, Dependency},
    entity::Entity,
    view::query::Query,
};

impl<Head> Query for Cons<Head, Nil>
where
    Head: Query,
{
    type Item<'item> = Cons<Head::Item<'item>, Nil>;

    type Fetch<'fetch> = Cons<Head::Fetch<'fetch>, Nil>;

    fn new<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let head = Head::new(components)?;
        let new_fetch = Cons(head, Nil);
        Some(new_fetch)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        let Cons(head, _) = fetch;
        let head = Head::fetch(head, entity)?;
        let item = Cons(head, Nil);
        Some(item)
    }
}

impl<Head, Tail> Query for Cons<Head, Tail>
where
    Head: Query,
    Tail: Query,
    for<'any> Head::Fetch<'any>: Dependency<&'any mut dyn Any>,
    for<'any> Tail::Fetch<'any>: Dependency<&'any mut dyn Any>,
{
    type Item<'item> = Cons<Head::Item<'item>, Tail::Item<'item>>;

    type Fetch<'fetch> = Cons<Head::Fetch<'fetch>, Tail::Fetch<'fetch>>;

    fn new<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let iter = components.iter_mut().map(AsAny::as_any_mut);
        dependency_from_iter(iter)
    }

    fn fetch<'borrow>(
        fetch: &'borrow mut Self::Fetch<'_>,
        entity: Entity,
    ) -> Option<Self::Item<'borrow>> {
        let Cons(head, tail) = fetch;
        let head = Head::fetch(head, entity)?;
        let tail = Tail::fetch(tail, entity)?;
        let item = Cons(head, tail);
        Some(item)
    }
}
