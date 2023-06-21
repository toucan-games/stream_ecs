use core::any::Any;

use hlist::{Cons, Nil};

pub trait RefMutContainer<'borrow>: Default {
    type RefMut: 'borrow;

    fn should_insert_any(&self, any: &dyn Any) -> bool;

    fn insert_any(&mut self, any: &'borrow mut dyn Any);

    fn into_ref_mut(self) -> Option<Self::RefMut>;
}

impl<'borrow, T> RefMutContainer<'borrow> for Option<&'borrow mut T>
where
    T: Any,
{
    type RefMut = &'borrow mut T;

    fn should_insert_any(&self, any: &dyn Any) -> bool {
        match self {
            Some(_) => false,
            None => any.is::<T>(),
        }
    }

    fn insert_any(&mut self, any: &'borrow mut dyn Any) {
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

impl<'borrow, Head> RefMutContainer<'borrow> for Cons<Head, Nil>
where
    Head: RefMutContainer<'borrow>,
{
    type RefMut = Cons<Head::RefMut, Nil>;

    fn should_insert_any(&self, any: &dyn Any) -> bool {
        let Cons(head, _) = self;
        head.should_insert_any(any)
    }

    fn insert_any(&mut self, any: &'borrow mut dyn Any) {
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

impl<'borrow, Head, Tail> RefMutContainer<'borrow> for Cons<Head, Tail>
where
    Head: RefMutContainer<'borrow>,
    Tail: RefMutContainer<'borrow>,
{
    type RefMut = Cons<Head::RefMut, Tail::RefMut>;

    fn should_insert_any(&self, any: &dyn Any) -> bool {
        let Cons(head, tail) = self;
        head.should_insert_any(any) || tail.should_insert_any(any)
    }

    fn insert_any(&mut self, any: &'borrow mut dyn Any) {
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
