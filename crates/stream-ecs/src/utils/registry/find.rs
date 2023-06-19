use core::any::Any;

use as_any::Downcast;
use hlist::{Cons, Nil};

pub trait Find: Any {
    fn find<T>(&self) -> Option<&T>
    where
        T: Any;

    fn find_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any;
}

impl Find for Nil {
    fn find<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        None
    }

    fn find_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any,
    {
        None
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
