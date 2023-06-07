use hlist::{ops::Get, Cons};

use crate::resource::Resource;

use super::{Provider, Registry};

use self::impl_details::{AsErased, AsErasedRefIter, AsErasedRefIterMut, Contains, Find, Len};

impl<Head, Tail> Registry for Cons<Head, Tail>
where
    Self: Len + Contains + Find + AsErased,
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
        Len::len(self)
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
    use as_any::AsAny;
    use hlist::{iter::Homogenous, Cons, Nil};

    use crate::resource::{ErasedResource, Resource};

    pub trait Len {
        fn len(&self) -> usize;
    }

    impl<T> Len for T
    where
        T: Resource,
    {
        fn len(&self) -> usize {
            1
        }
    }

    impl Len for Nil {
        fn len(&self) -> usize {
            0
        }
    }

    impl<Head, Tail> Len for Cons<Head, Tail>
    where
        Head: Len,
        Tail: Len,
    {
        fn len(&self) -> usize {
            let Cons(head, tail) = self;
            head.len() + tail.len()
        }
    }

    pub trait Contains {
        fn contains<R>(&self) -> bool
        where
            R: Resource;
    }

    impl<T> Contains for T
    where
        T: Resource,
    {
        fn contains<R>(&self) -> bool
        where
            R: Resource,
        {
            self.as_any().is::<R>()
        }
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
        Head: Contains,
        Tail: Contains,
    {
        fn contains<R>(&self) -> bool
        where
            R: Resource,
        {
            let Cons(head, tail) = self;
            head.contains::<R>() || tail.contains::<R>()
        }
    }

    pub trait Find {
        fn find<R>(&self) -> Option<&R>
        where
            R: Resource;

        fn find_mut<R>(&mut self) -> Option<&mut R>
        where
            R: Resource;
    }

    impl<T> Find for T
    where
        T: Resource,
    {
        fn find<R>(&self) -> Option<&R>
        where
            R: Resource,
        {
            self.as_any().downcast_ref()
        }

        fn find_mut<R>(&mut self) -> Option<&mut R>
        where
            R: Resource,
        {
            self.as_any_mut().downcast_mut()
        }
    }

    impl Find for Nil {
        fn find<R>(&self) -> Option<&R>
        where
            R: Resource,
        {
            None
        }

        fn find_mut<R>(&mut self) -> Option<&mut R>
        where
            R: Resource,
        {
            None
        }
    }

    impl<Head, Tail> Find for Cons<Head, Tail>
    where
        Head: Find,
        Tail: Find,
    {
        fn find<R>(&self) -> Option<&R>
        where
            R: Resource,
        {
            let Cons(head, tail) = self;
            match head.find() {
                Some(head) => Some(head),
                None => tail.find(),
            }
        }

        fn find_mut<R>(&mut self) -> Option<&mut R>
        where
            R: Resource,
        {
            let Cons(head, tail) = self;
            match head.find_mut() {
                Some(head) => Some(head),
                None => tail.find_mut(),
            }
        }
    }

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
