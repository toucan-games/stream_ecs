use hlist::{Cons, Nil};

use crate::entity::Entity;

use super::Fetch;

impl<Head> Fetch for Cons<Head, Nil>
where
    Head: Fetch,
{
    type Item<'a> = Cons<Head::Item<'a>, Nil>
    where
        Self: 'a;

    fn fetch(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        let Cons(head, _) = self;
        let head = head.fetch(entity)?;
        let item = Cons(head, Nil);
        Some(item)
    }
}

impl<Head, Tail> Fetch for Cons<Head, Tail>
where
    Head: Fetch,
    Tail: Fetch,
{
    type Item<'a> = Cons<Head::Item<'a>, Tail::Item<'a>>
    where
        Self: 'a;

    fn fetch(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        let Cons(head, tail) = self;
        let head = head.fetch(entity)?;
        let tail = tail.fetch(entity)?;
        let item = Cons(head, tail);
        Some(item)
    }
}
