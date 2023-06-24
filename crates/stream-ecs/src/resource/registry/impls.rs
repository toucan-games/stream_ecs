use hlist::{
    ops::{Get, Prepend},
    Cons, HList,
};

use crate::{
    resource::Resource,
    utils::registry::{Contains, Find},
};

use super::{Provider, Registry};

use self::impl_details::{AsErased, AsErasedRefIter, AsErasedRefIterMut};

impl<Head, Tail> Registry for Cons<Head, Tail>
where
    Self: Prepend + Contains + Find + AsErased,
    for<'any> <Self as AsErased>::Ref<'any>: AsErasedRefIter<'any>,
    for<'any> <Self as AsErased>::RefMut<'any>: AsErasedRefIterMut<'any>,
{
    type With<R> = <Self as Prepend>::Output<R>
    where
        R: Resource;

    fn with<R>(self, resource: R) -> Self::With<R>
    where
        R: Resource,
    {
        Prepend::prepend(self, resource)
    }

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
        T: Resource,
    {
        type Ref<'me> = &'me dyn ErasedResource
        where
            Self: 'me;

        fn as_erased(&self) -> Self::Ref<'_> {
            self
        }

        type RefMut<'me> = &'me mut dyn ErasedResource
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

    pub trait AsErasedRefIter<'item>: Homogenous<Item = &'item dyn ErasedResource> {}

    impl<'item, Tail> AsErasedRefIter<'item> for Cons<&'item dyn ErasedResource, Tail> where
        Self: Homogenous<Item = &'item dyn ErasedResource>
    {
    }

    pub trait AsErasedRefIterMut<'item>: Homogenous<Item = &'item mut dyn ErasedResource> {}

    impl<'item, Tail> AsErasedRefIterMut<'item> for Cons<&'item mut dyn ErasedResource, Tail> where
        Self: Homogenous<Item = &'item mut dyn ErasedResource>
    {
    }
}
