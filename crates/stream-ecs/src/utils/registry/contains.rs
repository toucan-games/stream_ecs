use core::any::Any;

use as_any::Downcast;
use hlist::{Cons, Nil};

pub trait Contains: Any {
    fn contains<T>(&self) -> bool
    where
        T: Any;
}

impl Contains for Nil {
    fn contains<T>(&self) -> bool
    where
        T: Any,
    {
        false
    }
}

impl<Head, Tail> Contains for Cons<Head, Tail>
where
    Head: Any,
    Tail: Contains,
{
    fn contains<T>(&self) -> bool
    where
        T: Any,
    {
        let Cons(head, tail) = self;
        head.is::<T>() || tail.contains::<T>()
    }
}
