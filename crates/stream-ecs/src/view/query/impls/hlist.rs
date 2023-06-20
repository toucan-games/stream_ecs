use hlist::{Cons, Nil};

use crate::{entity::Entity, view::query::Query};

impl<Head> Query for Cons<Head, Nil>
where
    Head: Query,
{
    type Item<'a> = Cons<Head::Item<'a>, Nil>;

    type Fetch<'a> = Cons<Head::Fetch<'a>, Nil>;

    fn fetch<'a>(fetch: &'a mut Self::Fetch<'_>, entity: Entity) -> Option<Self::Item<'a>> {
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
    type Item<'a> = Cons<Head::Item<'a>, Tail::Item<'a>>;

    type Fetch<'a> = Cons<Head::Fetch<'a>, Tail::Fetch<'a>>;

    fn fetch<'a>(fetch: &'a mut Self::Fetch<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        let Cons(head, tail) = fetch;
        let head = Head::fetch(head, entity)?;
        let tail = Tail::fetch(tail, entity)?;
        let item = Cons(head, tail);
        Some(item)
    }
}
