use hlist::{Cons, HList, Nil};

use crate::resource::{
    registry::{Registry as Resources, TryRegistry as TryResources},
    Resource,
};

use super::{Bundle, GetBundle, GetBundleMut, TryBundle};

/// Trivial implementation for resources, which forwards implementation to the resource registry.
impl<T> Bundle for T
where
    T: Resource,
{
    fn insert<R>(resources: &mut R, resource: Self) -> Option<Self>
    where
        R: Resources,
    {
        resources.insert(resource)
    }

    fn remove<R>(resources: &mut R) -> Option<Self>
    where
        R: Resources,
    {
        resources.remove()
    }

    fn contains<R>(resources: &R) -> bool
    where
        R: Resources,
    {
        resources.contains::<T>()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> Bundle for Cons<Head, Nil>
where
    Head: Bundle,
{
    fn insert<R>(resources: &mut R, bundle: Self) -> Option<Self>
    where
        R: Resources,
    {
        let Cons(head, nil) = bundle;
        let head = Head::insert(resources, head)?;
        let bundle = Cons(head, nil);
        Some(bundle)
    }

    fn remove<R>(resources: &mut R) -> Option<Self>
    where
        R: Resources,
    {
        let head = Head::remove(resources)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }

    fn contains<R>(resources: &R) -> bool
    where
        R: Resources,
    {
        Head::contains(resources)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> Bundle for Cons<Head, Tail>
where
    Head: Bundle,
    Tail: Bundle + HList,
{
    fn insert<R>(resources: &mut R, bundle: Self) -> Option<Self>
    where
        R: Resources,
    {
        let Cons(head, tail) = bundle;
        let head = Head::insert(resources, head)?;
        let tail = Tail::insert(resources, tail)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }

    fn remove<R>(resources: &mut R) -> Option<Self>
    where
        R: Resources,
    {
        let head = Head::remove(resources)?;
        let tail = Tail::remove(resources)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }

    fn contains<R>(resources: &R) -> bool
    where
        R: Resources,
    {
        Head::contains(resources) && Tail::contains(resources)
    }
}

/// Trivial implementation for resources, which forwards implementation to the resource registry.
impl<T> TryBundle for T
where
    T: Resource,
{
    fn try_insert<R>(resources: &mut R, resource: Self) -> Result<Option<Self>, R::Err>
    where
        R: TryResources,
    {
        resources.try_insert(resource)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> TryBundle for Cons<Head, Nil>
where
    Head: TryBundle,
{
    fn try_insert<R>(resources: &mut R, bundle: Self) -> Result<Option<Self>, R::Err>
    where
        R: TryResources,
    {
        let Cons(head, nil) = bundle;
        let Some(head) = Head::try_insert(resources, head)? else {
            return Ok(None);
        };
        let bundle = Cons(head, nil);
        Ok(Some(bundle))
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> TryBundle for Cons<Head, Tail>
where
    Head: TryBundle,
    Tail: TryBundle + HList,
{
    fn try_insert<R>(resources: &mut R, bundle: Self) -> Result<Option<Self>, R::Err>
    where
        R: TryResources,
    {
        let Cons(head, tail) = bundle;
        let Some(head) = Head::try_insert(resources, head)? else {
            return Ok(None);
        };
        let Some(tail) = Tail::try_insert(resources, tail)? else {
            return Ok(None);
        };
        let bundle = Cons(head, tail);
        Ok(Some(bundle))
    }
}

/// Trivial implementation for resources, which forwards implementation to the resource registry.
impl<T> GetBundle for T
where
    T: Resource,
{
    type Ref<'a> = &'a T
    where
        Self: 'a;

    fn get<R>(resources: &R) -> Option<Self::Ref<'_>>
    where
        R: Resources,
    {
        resources.get()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetBundle for Cons<Head, Nil>
where
    Head: GetBundle,
{
    type Ref<'a> = Cons<Head::Ref<'a>, Nil>
    where
        Self: 'a;

    fn get<R>(resources: &R) -> Option<Self::Ref<'_>>
    where
        R: Resources,
    {
        let head = Head::get(resources)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> GetBundle for Cons<Head, Tail>
where
    Head: GetBundle,
    Tail: GetBundle + HList,
    for<'a> Tail::Ref<'a>: HList,
{
    type Ref<'a> = Cons<Head::Ref<'a>, Tail::Ref<'a>>
    where
        Self: 'a;

    fn get<R>(resources: &R) -> Option<Self::Ref<'_>>
    where
        R: Resources,
    {
        let head = Head::get(resources)?;
        let tail = Tail::get(resources)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }
}

/// Trivial implementation for resources, which forwards implementation to the resource registry.
impl<T> GetBundleMut for T
where
    T: Resource,
{
    type RefMut<'a> = &'a mut T
    where
        Self: 'a;

    fn get_mut<R>(resources: &mut R) -> Option<Self::RefMut<'_>>
    where
        R: Resources,
    {
        resources.get_mut()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetBundleMut for Cons<Head, Nil>
where
    Head: GetBundleMut,
{
    type RefMut<'a> = Cons<Head::RefMut<'a>, Nil>
    where
        Self: 'a;

    fn get_mut<R>(resources: &mut R) -> Option<Self::RefMut<'_>>
    where
        R: Resources,
    {
        let head = Head::get_mut(resources)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }
}

use crate::ref_mut::{RefMut, RefMutContainer as Rmc};

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> GetBundleMut for Cons<Head, Tail>
where
    Head: GetBundleMut,
    Tail: GetBundleMut + HList,
    for<'a> Head::RefMut<'a>: RefMut<'a>,
    for<'a> Tail::RefMut<'a>: RefMut<'a> + HList,
    for<'a> <Head::RefMut<'a> as RefMut<'a>>::Container: Rmc<'a, RefMut = Head::RefMut<'a>>,
    for<'a> <Tail::RefMut<'a> as RefMut<'a>>::Container: Rmc<'a, RefMut = Tail::RefMut<'a>> + HList,
{
    type RefMut<'a> = Cons<Head::RefMut<'a>, Tail::RefMut<'a>>
    where
        Self: 'a;

    fn get_mut<R>(resources: &mut R) -> Option<Self::RefMut<'_>>
    where
        R: Resources,
    {
        type Container<'a, T> = <T as RefMut<'a>>::Container;

        let mut container: Container<Self::RefMut<'_>> = Default::default();
        for resource in resources.iter_mut() {
            let any = resource.as_any_mut();
            container.insert_any(any);
        }
        container.into_ref_mut()
    }
}
