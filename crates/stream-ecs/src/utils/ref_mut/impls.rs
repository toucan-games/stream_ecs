use core::any::Any;

use hlist::{Cons, Nil};

use super::Dependency;

impl<'me, T> Dependency<&'me mut dyn Any> for &'me mut T
where
    T: Any,
{
    type Container = Option<&'me mut T>;
}

impl<Input> Dependency<Input> for Nil {
    type Container = Self;
}

impl<Input, Head, Tail> Dependency<Input> for Cons<Head, Tail>
where
    Head: Dependency<Input>,
    Tail: Dependency<Input>,
{
    type Container = Cons<Head::Container, Tail::Container>;
}
