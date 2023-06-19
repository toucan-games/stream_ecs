use hlist::{ops::Get, Cons, HList};

use crate::{
    resource::Resource,
    utils::registry::{Contains, Find},
};

use super::{Provider, Registry};

use self::impl_details::{AsErased, AsErasedRefIter, AsErasedRefIterMut};

impl<Head, Tail> Registry for Cons<Head, Tail>
where
    Self: HList + Contains + Find + AsErased,
    for<'a> <Self as AsErased>::Ref<'a>: AsErasedRefIter<'a>,
    for<'a> <Self as AsErased>::RefMut<'a>: AsErasedRefIterMut<'a>,
{
    fn contains<R>(&self) -> bool
    where
        R: Resource,
    {
        Contains::contains::<R>(self)
    }

    fn len(&self) -> usize {
        HList::len(self)
    }

    fn get<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        Find::find(self)
    }

    fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        Find::find_mut(self)
    }

    type Iter<'a> = <<Self as AsErased>::Ref<'a> as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        let erased = AsErased::as_erased(self);
        erased.into_iter()
    }

    type IterMut<'a> = <<Self as AsErased>::RefMut<'a> as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let erased = AsErased::as_erased_mut(self);
        erased.into_iter()
    }
}

impl<R, I, T> Provider<R, I> for T
where
    T: Registry + Get<R, I>,
    R: Resource,
{
    fn provide(&self) -> &R {
        Get::get(self)
    }

    fn provide_mut(&mut self) -> &mut R {
        Get::get_mut(self)
    }
}

mod impl_details {
    use hlist::{iter::Homogenous, Cons, Nil};

    use crate::resource::{ErasedResource, Resource};

    pub trait AsErased {
        type Ref<'a>
        where
            Self: 'a;

        fn as_erased(&self) -> Self::Ref<'_>;

        type RefMut<'a>
        where
            Self: 'a;

        fn as_erased_mut(&mut self) -> Self::RefMut<'_>;
    }

    impl<T> AsErased for T
    where
        T: Resource,
    {
        type Ref<'a> = &'a dyn ErasedResource
        where
            Self: 'a;

        fn as_erased(&self) -> Self::Ref<'_> {
            self
        }

        type RefMut<'a> = &'a mut dyn ErasedResource
        where
            Self: 'a;

        fn as_erased_mut(&mut self) -> Self::RefMut<'_> {
            self
        }
    }

    impl<Head> AsErased for Cons<Head, Nil>
    where
        Head: AsErased,
    {
        type Ref<'a> = Cons<Head::Ref<'a>, Nil>
        where
            Self: 'a;

        fn as_erased(&self) -> Self::Ref<'_> {
            let Cons(head, _) = self;
            let head = head.as_erased();
            Cons(head, Nil)
        }

        type RefMut<'a> = Cons<Head::RefMut<'a>, Nil>
        where
            Self: 'a;

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
        type Ref<'a> = Cons<Head::Ref<'a>, Tail::Ref<'a>>
        where
            Self: 'a;

        fn as_erased(&self) -> Self::Ref<'_> {
            let Cons(head, tail) = self;
            let head = head.as_erased();
            let tail = tail.as_erased();
            Cons(head, tail)
        }

        type RefMut<'a> = Cons<Head::RefMut<'a>, Tail::RefMut<'a>>
        where
            Self: 'a;

        fn as_erased_mut(&mut self) -> Self::RefMut<'_> {
            let Cons(head, tail) = self;
            let head = head.as_erased_mut();
            let tail = tail.as_erased_mut();
            Cons(head, tail)
        }
    }

    pub trait AsErasedRefIter<'a>: Homogenous<Item = &'a dyn ErasedResource> {}

    impl<'a, Tail> AsErasedRefIter<'a> for Cons<&'a dyn ErasedResource, Tail> where
        Self: Homogenous<Item = &'a dyn ErasedResource>
    {
    }

    pub trait AsErasedRefIterMut<'a>: Homogenous<Item = &'a mut dyn ErasedResource> {}

    impl<'a, Tail> AsErasedRefIterMut<'a> for Cons<&'a mut dyn ErasedResource, Tail> where
        Self: Homogenous<Item = &'a mut dyn ErasedResource>
    {
    }
}
