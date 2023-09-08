use hlist::{
    ops::{Get, Index, Prepend},
    Cons, HList,
};

use crate::{
    component::Component,
    utils::registry::{Contains, Find},
};

use super::{Provider, Registry, With};

use self::impl_details::{AsErased, AsErasedRefIter, AsErasedRefIterMut};

impl<Head, Tail> Registry for Cons<Head, Tail>
where
    Self: Prepend + Contains + Find + AsErased,
    for<'any> <Self as AsErased>::Ref<'any>: AsErasedRefIter<'any>,
    for<'any> <Self as AsErased>::RefMut<'any>: AsErasedRefIterMut<'any>,
{
    fn is_registered<C>(&self) -> bool
    where
        C: Component,
    {
        Contains::contains::<C::Storage>(self)
    }

    fn len(&self) -> usize {
        HList::len(self)
    }

    fn get<C>(&self) -> Option<&C::Storage>
    where
        C: Component,
    {
        Find::find(self)
    }

    fn get_mut<C>(&mut self) -> Option<&mut C::Storage>
    where
        C: Component,
    {
        Find::find_mut(self)
    }

    type Iter<'me> = <<Self as AsErased>::Ref<'me> as IntoIterator>::IntoIter
    where
        Self: 'me;

    fn iter(&self) -> Self::Iter<'_> {
        let erased = AsErased::as_erased(self);
        erased.into_iter()
    }

    type IterMut<'me> = <<Self as AsErased>::RefMut<'me> as IntoIterator>::IntoIter
    where
        Self: 'me;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let erased = AsErased::as_erased_mut(self);
        erased.into_iter()
    }
}

impl<Head, Tail> With for Cons<Head, Tail>
where
    Self: Prepend + Contains + Find + AsErased,
    for<'any> <Self as AsErased>::Ref<'any>: AsErasedRefIter<'any>,
    for<'any> <Self as AsErased>::RefMut<'any>: AsErasedRefIterMut<'any>,
{
    type Output<C> = <Self as Prepend>::Output<C::Storage>
    where
        C: Component;

    fn with<C>(self, storage: C::Storage) -> Self::Output<C>
    where
        C: Component,
    {
        Prepend::prepend(self, storage)
    }
}

impl<C, I, T> Provider<C, I> for T
where
    T: Registry + Get<C::Storage, I>,
    I: Index,
    C: Component,
{
    fn provide(&self) -> &C::Storage {
        Get::get(self)
    }

    fn provide_mut(&mut self) -> &mut C::Storage {
        Get::get_mut(self)
    }
}

mod impl_details {
    use hlist::{iter::Homogenous, Cons, Nil};

    use crate::component::storage::{ErasedStorage, Storage};

    pub trait AsErased {
        type Ref<'me>
        where
            Self: 'me;

        fn as_erased(&self) -> Self::Ref<'_>;

        type RefMut<'me>
        where
            Self: 'me;

        fn as_erased_mut(&mut self) -> Self::RefMut<'_>;
    }

    impl<T> AsErased for T
    where
        T: Storage,
    {
        type Ref<'me> = &'me dyn ErasedStorage
        where
            Self: 'me;

        fn as_erased(&self) -> Self::Ref<'_> {
            self
        }

        type RefMut<'me> = &'me mut dyn ErasedStorage
        where
            Self: 'me;

        fn as_erased_mut(&mut self) -> Self::RefMut<'_> {
            self
        }
    }

    impl<Head> AsErased for Cons<Head, Nil>
    where
        Head: AsErased,
    {
        type Ref<'me> = Cons<Head::Ref<'me>, Nil>
        where
            Self: 'me;

        fn as_erased(&self) -> Self::Ref<'_> {
            let Cons(head, _) = self;
            let head = head.as_erased();
            Cons(head, Nil)
        }

        type RefMut<'me> = Cons<Head::RefMut<'me>, Nil>
        where
            Self: 'me;

        fn as_erased_mut(&mut self) -> Self::RefMut<'_> {
            let Cons(head, _) = self;
            let head = head.as_erased_mut();
            Cons(head, Nil)
        }
    }

    impl<Head, Tail> AsErased for Cons<Head, Tail>
    where
        Head: AsErased,
        Tail: AsErased,
    {
        type Ref<'me> = Cons<Head::Ref<'me>, Tail::Ref<'me>>
        where
            Self: 'me;

        fn as_erased(&self) -> Self::Ref<'_> {
            let Cons(head, tail) = self;
            let head = head.as_erased();
            let tail = tail.as_erased();
            Cons(head, tail)
        }

        type RefMut<'me> = Cons<Head::RefMut<'me>, Tail::RefMut<'me>>
        where
            Self: 'me;

        fn as_erased_mut(&mut self) -> Self::RefMut<'_> {
            let Cons(head, tail) = self;
            let head = head.as_erased_mut();
            let tail = tail.as_erased_mut();
            Cons(head, tail)
        }
    }

    pub trait AsErasedRefIter<'item>: Homogenous<Item = &'item dyn ErasedStorage> {}

    impl<'item, Tail> AsErasedRefIter<'item> for Cons<&'item dyn ErasedStorage, Tail> where
        Self: Homogenous<Item = &'item dyn ErasedStorage>
    {
    }

    pub trait AsErasedRefIterMut<'item>: Homogenous<Item = &'item mut dyn ErasedStorage> {}

    impl<'item, Tail> AsErasedRefIterMut<'item> for Cons<&'item mut dyn ErasedStorage, Tail> where
        Self: Homogenous<Item = &'item mut dyn ErasedStorage>
    {
    }
}
