use hlist::{Cons, Nil};

use super::Fetch;

impl<'a, Head> Fetch<'a> for Cons<Head, Nil>
where
    Head: Fetch<'a>,
{
    type Item = Cons<Head::Item, Nil>;
}

impl<'a, Head, Tail> Fetch<'a> for Cons<Head, Tail>
where
    Head: Fetch<'a>,
    Tail: Fetch<'a>,
{
    type Item = Cons<Head::Item, Tail::Item>;
}
