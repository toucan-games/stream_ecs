use core::any::Any;

use hlist::{Cons, Nil};

use super::RefMut;

impl<'a, T> RefMut<'a> for &'a mut T
where
    T: Any,
{
    type Container = Option<&'a mut T>;
}

impl<'a, Head> RefMut<'a> for Cons<Head, Nil>
where
    Head: RefMut<'a>,
{
    type Container = Cons<Head::Container, Nil>;
}

impl<'a, Head, Tail> RefMut<'a> for Cons<Head, Tail>
where
    Head: RefMut<'a>,
    Tail: RefMut<'a>,
{
    type Container = Cons<Head::Container, Tail::Container>;
}
