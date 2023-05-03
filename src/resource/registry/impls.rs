use hlist::{ops::Get, Cons, HList};

use crate::resource::Resource;

use super::{Provider, Registry};

use self::impl_details::{AsErased, AsErasedMut, Contains, Find, FindMut};

impl<Head, Tail> Registry for Cons<Head, Tail>
where
    Head: Resource,
    Tail: Send + Sync,
    Cons<Head, Tail>: Contains + Find + FindMut + AsErased + AsErasedMut,
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
        FindMut::find_mut(self)
    }

    type Iter<'a> = <<Self as AsErased>::Output<'a> as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        let erased = AsErased::as_erased(self);
        erased.into_iter()
    }

    type IterMut<'a> = <<Self as AsErasedMut>::Output<'a> as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let erased = AsErasedMut::as_erased_mut(self);
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
    use as_any::AsAny;
    use hlist::{Cons, HList, Nil};

    use crate::resource::{ErasedResource, Resource};

    pub trait Contains: HList {
        fn contains<R>(&self) -> bool
        where
            R: Resource;
    }

    impl Contains for Nil {
        fn contains<R>(&self) -> bool
        where
            R: Resource,
        {
            false
        }
    }

    impl<Head, Tail> Contains for Cons<Head, Tail>
    where
        Head: Resource,
        Tail: Contains,
    {
        fn contains<R>(&self) -> bool
        where
            R: Resource,
        {
            let Cons(head, tail) = self;
            let in_head = head.as_any().is::<R>();
            in_head || tail.contains::<R>()
        }
    }

    pub trait Find: HList {
        fn find<R>(&self) -> Option<&R>
        where
            R: Resource;
    }

    impl Find for Nil {
        fn find<R>(&self) -> Option<&R>
        where
            R: Resource,
        {
            None
        }
    }

    impl<Head, Tail> Find for Cons<Head, Tail>
    where
        Head: Resource,
        Tail: Find,
    {
        fn find<R>(&self) -> Option<&R>
        where
            R: Resource,
        {
            let Cons(head, tail) = self;
            let head = head.as_any().downcast_ref();
            match head {
                Some(head) => Some(head),
                None => tail.find(),
            }
        }
    }

    pub trait FindMut: HList {
        fn find_mut<R>(&mut self) -> Option<&mut R>
        where
            R: Resource;
    }

    impl FindMut for Nil {
        fn find_mut<R>(&mut self) -> Option<&mut R>
        where
            R: Resource,
        {
            None
        }
    }

    impl<Head, Tail> FindMut for Cons<Head, Tail>
    where
        Head: Resource,
        Tail: FindMut,
    {
        fn find_mut<R>(&mut self) -> Option<&mut R>
        where
            R: Resource,
        {
            let Cons(head, tail) = self;
            let head = head.as_any_mut().downcast_mut();
            match head {
                Some(head) => Some(head),
                None => tail.find_mut(),
            }
        }
    }

    pub trait AsErased: HList {
        type Output<'a>: HList + IntoIterator<Item = &'a dyn ErasedResource>
        where
            Self: 'a;

        fn as_erased(&self) -> Self::Output<'_>;
    }

    impl<Head> AsErased for Cons<Head, Nil>
    where
        Head: Resource,
    {
        type Output<'a> = Cons<&'a dyn ErasedResource, Nil>
        where
            Self: 'a;

        fn as_erased(&self) -> Self::Output<'_> {
            let Cons(head, _) = self;
            let head = head as _;
            Cons(head, Nil)
        }
    }

    impl<Head, Tail> AsErased for Cons<Head, Tail>
    where
        Head: Resource,
        Tail: AsErased,
        for<'a> Cons<&'a dyn ErasedResource, Tail::Output<'a>>:
            IntoIterator<Item = &'a dyn ErasedResource>,
    {
        type Output<'a> = Cons<&'a dyn ErasedResource, Tail::Output<'a>>
        where
            Self: 'a;

        fn as_erased(&self) -> Self::Output<'_> {
            let Cons(head, tail) = self;
            let head = head as _;
            let tail = tail.as_erased();
            Cons(head, tail)
        }
    }

    pub trait AsErasedMut: HList {
        type Output<'a>: HList + IntoIterator<Item = &'a mut dyn ErasedResource>
        where
            Self: 'a;

        fn as_erased_mut(&mut self) -> Self::Output<'_>;
    }

    impl<Head> AsErasedMut for Cons<Head, Nil>
    where
        Head: Resource,
    {
        type Output<'a> = Cons<&'a mut dyn ErasedResource, Nil>
        where
            Self: 'a;

        fn as_erased_mut(&mut self) -> Self::Output<'_> {
            let Cons(head, _) = self;
            let head = head as _;
            Cons(head, Nil)
        }
    }

    impl<Head, Tail> AsErasedMut for Cons<Head, Tail>
    where
        Head: Resource,
        Tail: AsErasedMut,
        for<'a> Cons<&'a mut dyn ErasedResource, Tail::Output<'a>>:
            IntoIterator<Item = &'a mut dyn ErasedResource>,
    {
        type Output<'a> = Cons<&'a mut dyn ErasedResource, Tail::Output<'a>>
        where
            Self: 'a;

        fn as_erased_mut(&mut self) -> Self::Output<'_> {
            let Cons(head, tail) = self;
            let head = head as _;
            let tail = tail.as_erased_mut();
            Cons(head, tail)
        }
    }
}
