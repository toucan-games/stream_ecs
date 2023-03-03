use core::any::{Any, TypeId};

use hlist::{Cons, HList, Nil};

pub trait RefMut<'a> {
    type Container: RefMutContainer<'a, RefMut = Self>;
}

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
    Tail: RefMut<'a> + HList,
    Tail::Container: HList,
{
    type Container = Cons<Head::Container, Tail::Container>;
}

pub trait RefMutContainer<'a>: Default {
    type RefMut: 'a;

    fn should_insert_any(&self, any: &dyn Any) -> bool;

    fn insert_any(&mut self, any: &'a mut dyn Any);

    fn into_ref_mut(self) -> Option<Self::RefMut>;
}

impl<'a, T> RefMutContainer<'a> for Option<&'a mut T>
where
    T: Any,
{
    type RefMut = &'a mut T;

    fn should_insert_any(&self, any: &dyn Any) -> bool {
        match self {
            Some(_) => false,
            None => any.type_id() == TypeId::of::<T>(),
        }
    }

    fn insert_any(&mut self, any: &'a mut dyn Any) {
        if self.is_some() {
            return;
        }
        let Some(ref_mut) = any.downcast_mut() else {
            return;
        };
        *self = Some(ref_mut);
    }

    fn into_ref_mut(self) -> Option<Self::RefMut> {
        self
    }
}

impl<'a, Head> RefMutContainer<'a> for Cons<Head, Nil>
where
    Head: RefMutContainer<'a>,
{
    type RefMut = Cons<Head::RefMut, Nil>;

    fn should_insert_any(&self, any: &dyn Any) -> bool {
        let Cons(head, _) = self;
        head.should_insert_any(any)
    }

    fn insert_any(&mut self, any: &'a mut dyn Any) {
        let Cons(head, _) = self;
        head.insert_any(any)
    }

    fn into_ref_mut(self) -> Option<Self::RefMut> {
        let Cons(head, nil) = self;
        let head = head.into_ref_mut()?;
        let ref_mut = Cons(head, nil);
        Some(ref_mut)
    }
}

impl<'a, Head, Tail> RefMutContainer<'a> for Cons<Head, Tail>
where
    Head: RefMutContainer<'a>,
    Tail: RefMutContainer<'a> + HList,
    Tail::RefMut: HList,
{
    type RefMut = Cons<Head::RefMut, Tail::RefMut>;

    fn should_insert_any(&self, any: &dyn Any) -> bool {
        let Cons(head, tail) = self;
        head.should_insert_any(any) || tail.should_insert_any(any)
    }

    fn insert_any(&mut self, any: &'a mut dyn Any) {
        let Cons(head, tail) = self;
        if head.should_insert_any(any) {
            head.insert_any(any);
            return;
        }
        tail.insert_any(any)
    }

    fn into_ref_mut(self) -> Option<Self::RefMut> {
        let Cons(head, tail) = self;
        let head = head.into_ref_mut()?;
        let tail = tail.into_ref_mut()?;
        let ref_mut = Cons(head, tail);
        Some(ref_mut)
    }
}
