use hlist::{Cons, Nil};

use crate::{component::registry::Registry as Components, entity::Entity, view::query::Query};

impl<Head> Query for Cons<Head, Nil>
where
    Head: Query,
{
    type Item<'item> = Cons<Head::Item<'item>, Nil>;

    type Fetch<'fetch> = Cons<Head::Fetch<'fetch>, Nil>;

    fn new_fetch<C>(components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        let head = Head::new_fetch(components)?;
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
{
    type Item<'item> = Cons<Head::Item<'item>, Tail::Item<'item>>;

    type Fetch<'fetch> = Cons<Head::Fetch<'fetch>, Tail::Fetch<'fetch>>;

    fn new_fetch<C>(_components: &mut C) -> Option<Self::Fetch<'_>>
    where
        C: Components,
    {
        // TODO collect dependencies, then create fetches from them
        None
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
