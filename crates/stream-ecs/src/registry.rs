use core::any::Any;

use as_any::Downcast;
use hlist::{Cons, Nil};

pub trait Contains: Any {
    fn contains<T>(&self) -> bool
    where
        T: Any;
}

impl<Head> Contains for Cons<Head, Nil>
where
    Head: Any,
{
    fn contains<T>(&self) -> bool
    where
        T: Any,
    {
        let Cons(head, _) = self;
        head.is::<T>()
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

pub trait Find: Any {
    fn find<T>(&self) -> Option<&T>
    where
        T: Any;

    fn find_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any;
}

impl<Head> Find for Cons<Head, Nil>
where
    Head: Any,
{
    fn find<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        let Cons(head, _) = self;
        head.downcast_ref()
    }

    fn find_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any,
    {
        let Cons(head, _) = self;
        head.downcast_mut()
    }
}

impl<Head, Tail> Find for Cons<Head, Tail>
where
    Head: Any,
    Tail: Find,
{
    fn find<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        let Cons(head, tail) = self;
        match head.downcast_ref() {
            Some(head) => Some(head),
            None => tail.find(),
        }
    }

    fn find_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any,
    {
        let Cons(head, tail) = self;
        match head.downcast_mut() {
            Some(head) => Some(head),
            None => tail.find_mut(),
        }
    }
}
