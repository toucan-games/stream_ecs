use hlist::{Cons, HList, Nil};

use crate::{
    component::{
        registry::{Registry as Components, TryRegistry as TryComponents},
        storage::Storage,
    },
    ref_mut::{RefMut, RefMutContainer},
};

use super::{Bundle, GetBundle, GetBundleMut, TryBundle};

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> Bundle for T
where
    T: Storage,
{
    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: Components,
    {
        components.register::<T::Item>(bundle)
    }

    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: Components,
    {
        components.unregister::<T::Item>()
    }

    fn is_registered<C>(components: &C) -> bool
    where
        C: Components,
    {
        components.is_registered::<T::Item>()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> Bundle for Cons<Head, Nil>
where
    Head: Bundle,
{
    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: Components,
    {
        let Cons(head, nil) = bundle;
        let head = Head::register(components, head)?;
        let bundle = Cons(head, nil);
        Some(bundle)
    }

    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: Components,
    {
        let head = Head::unregister(components)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }

    fn is_registered<C>(components: &C) -> bool
    where
        C: Components,
    {
        Head::is_registered(components)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> Bundle for Cons<Head, Tail>
where
    Head: Bundle,
    Tail: Bundle + HList,
{
    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: Components,
    {
        let Cons(head, tail) = bundle;
        let head = Head::register(components, head)?;
        let tail = Tail::register(components, tail)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }

    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: Components,
    {
        let head = Head::unregister(components)?;
        let tail = Tail::unregister(components)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }

    fn is_registered<C>(components: &C) -> bool
    where
        C: Components,
    {
        Head::is_registered(components) && Tail::is_registered(components)
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> TryBundle for T
where
    T: Storage,
{
    fn try_register<C>(components: &mut C, bundle: Self) -> Result<Option<Self>, C::Err>
    where
        C: TryComponents,
    {
        components.try_register::<T::Item>(bundle)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> TryBundle for Cons<Head, Nil>
where
    Head: TryBundle,
{
    fn try_register<C>(components: &mut C, bundle: Self) -> Result<Option<Self>, C::Err>
    where
        C: TryComponents,
    {
        let Cons(head, nil) = bundle;
        let Some(head) = Head::try_register(components, head)? else {
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
    fn try_register<C>(components: &mut C, bundle: Self) -> Result<Option<Self>, C::Err>
    where
        C: TryComponents,
    {
        let Cons(head, tail) = bundle;
        let Some(head) = Head::try_register(components, head)? else {
                return Ok(None);
            };
        let Some(tail) = Tail::try_register(components, tail)? else {
                return Ok(None);
            };
        let bundle = Cons(head, tail);
        Ok(Some(bundle))
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> GetBundle for T
where
    T: Storage,
{
    type Ref<'a> = &'a T
    where
        Self: 'a;

    fn get<C>(components: &C) -> Option<Self::Ref<'_>>
    where
        C: Components,
    {
        components.get::<T::Item>()
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

    fn get<C>(components: &C) -> Option<Self::Ref<'_>>
    where
        C: Components,
    {
        let head = Head::get(components)?;
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

    fn get<C>(components: &C) -> Option<Self::Ref<'_>>
    where
        C: Components,
    {
        let head = Head::get(components)?;
        let tail = Tail::get(components)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> GetBundleMut for T
where
    T: Storage,
{
    type RefMut<'a> = &'a mut T
    where
        Self: 'a;

    fn get_mut<C>(components: &mut C) -> Option<Self::RefMut<'_>>
    where
        C: Components,
    {
        components.get_mut::<T::Item>()
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

    fn get_mut<C>(components: &mut C) -> Option<Self::RefMut<'_>>
    where
        C: Components,
    {
        let head = Head::get_mut(components)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> GetBundleMut for Cons<Head, Tail>
where
    Head: GetBundleMut,
    Tail: GetBundleMut + HList,
    for<'a> Head::RefMut<'a>: RefMut<'a>,
    for<'a> Tail::RefMut<'a>: RefMut<'a> + HList,
    for<'a> <Tail::RefMut<'a> as RefMut<'a>>::Container: HList,
{
    type RefMut<'a> = Cons<Head::RefMut<'a>, Tail::RefMut<'a>>
    where
        Self: 'a;

    fn get_mut<C>(components: &mut C) -> Option<Self::RefMut<'_>>
    where
        C: Components,
    {
        type Container<'a, T> = <T as RefMut<'a>>::Container;

        let mut container: Container<Self::RefMut<'_>> = Default::default();
        for storage in components.iter_mut() {
            let any = storage.as_any_mut();
            container.insert_any(any);
        }
        container.into_ref_mut()
    }
}
