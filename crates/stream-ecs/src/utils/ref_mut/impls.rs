use core::any::Any;

use hlist::{Cons, Nil};

use super::RefMut;

impl<'borrow, T> RefMut<'borrow> for &'borrow mut T
where
    T: Any,
{
    type Container = Option<&'borrow mut T>;
}

impl<'borrow, Head> RefMut<'borrow> for Cons<Head, Nil>
where
    Head: RefMut<'borrow>,
{
    type Container = Cons<Head::Container, Nil>;
}

impl<'borrow, Head, Tail> RefMut<'borrow> for Cons<Head, Tail>
where
    Head: RefMut<'borrow>,
    Tail: RefMut<'borrow>,
{
    type Container = Cons<Head::Container, Tail::Container>;
}
