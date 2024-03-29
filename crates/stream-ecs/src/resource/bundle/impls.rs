use core::any::Any;

use as_any::AsAny;
use hlist::{Cons, Nil};

use crate::{
    dependency::{dependency_from_iter, Dependency},
    resource::{
        registry::{
            Provider as ResourcesProvider, Registry as Resources, RegistryMut as ResourcesMut,
            TryRegistryMut as TryResourcesMut, With as WithResources,
        },
        Resource,
    },
};

use super::{Bundle, GetBundle, GetBundleMut, ProvideBundle, ProvideBundleMut, TryBundle};

/// Trivial implementation for resources, which forwards implementation to the resource registry.
impl<T> Bundle for T
where
    T: Resource,
{
    type With<R> = R::Output<T>
    where
        R: WithResources;

    fn with<R>(resources: R, bundle: Self) -> Self::With<R>
    where
        R: WithResources,
    {
        resources.with(bundle)
    }

    fn insert<R>(resources: &mut R, resource: Self) -> Option<Self>
    where
        R: ResourcesMut,
    {
        resources.insert(resource)
    }

    fn remove<R>(resources: &mut R) -> Option<Self>
    where
        R: ResourcesMut,
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
    type With<R> = Cons<Head::With<R>, Nil>
    where
        R: WithResources;

    fn with<R>(resources: R, bundle: Self) -> Self::With<R>
    where
        R: WithResources,
    {
        let Cons(head, nil) = bundle;
        let head = Head::with(resources, head);
        Cons(head, nil)
    }

    fn insert<R>(resources: &mut R, bundle: Self) -> Option<Self>
    where
        R: ResourcesMut,
    {
        let Cons(head, nil) = bundle;
        let head = Head::insert(resources, head)?;
        let bundle = Cons(head, nil);
        Some(bundle)
    }

    fn remove<R>(resources: &mut R) -> Option<Self>
    where
        R: ResourcesMut,
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
    Tail: Bundle,
{
    type With<R> = Cons<Head::With<R>, Tail>
    where
        R: WithResources;

    fn with<R>(resources: R, bundle: Self) -> Self::With<R>
    where
        R: WithResources,
    {
        let Cons(head, tail) = bundle;
        let head = Head::with(resources, head);
        Cons(head, tail)
    }

    fn insert<R>(resources: &mut R, bundle: Self) -> Option<Self>
    where
        R: ResourcesMut,
    {
        let Cons(head, tail) = bundle;
        let head = Head::insert(resources, head)?;
        let tail = Tail::insert(resources, tail)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }

    fn remove<R>(resources: &mut R) -> Option<Self>
    where
        R: ResourcesMut,
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
        R: TryResourcesMut,
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
        R: TryResourcesMut,
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
    Tail: TryBundle,
{
    fn try_insert<R>(resources: &mut R, bundle: Self) -> Result<Option<Self>, R::Err>
    where
        R: TryResourcesMut,
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
    type Ref<'resources> = &'resources T;

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
    type Ref<'resources> = Cons<Head::Ref<'resources>, Nil>;

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
    Tail: GetBundle,
{
    type Ref<'resources> = Cons<Head::Ref<'resources>, Tail::Ref<'resources>>;

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
    type RefMut<'resources> = &'resources mut T;

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
    type RefMut<'resources> = Cons<Head::RefMut<'resources>, Nil>;

    fn get_mut<R>(resources: &mut R) -> Option<Self::RefMut<'_>>
    where
        R: Resources,
    {
        let head = Head::get_mut(resources)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> GetBundleMut for Cons<Head, Tail>
where
    Head: GetBundleMut,
    Tail: GetBundleMut,
    for<'any> Head::RefMut<'any>: Dependency<&'any mut dyn Any>,
    for<'any> Tail::RefMut<'any>: Dependency<&'any mut dyn Any>,
{
    type RefMut<'resources> = Cons<Head::RefMut<'resources>, Tail::RefMut<'resources>>;

    fn get_mut<R>(resources: &mut R) -> Option<Self::RefMut<'_>>
    where
        R: Resources,
    {
        let iter = resources.iter_mut().map(AsAny::as_any_mut);
        dependency_from_iter(iter).ok()
    }
}

/// Trivial implementation for resources, which forwards implementation to the resource registry.
impl<T, R, I> ProvideBundle<R, I> for T
where
    T: Resource,
    R: ResourcesProvider<T, I>,
{
    type Ref<'resources> = &'resources T
    where
        R: 'resources;

    fn provide(resources: &R) -> Self::Ref<'_> {
        resources.provide()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head, R, I> ProvideBundle<R, I> for Cons<Head, Nil>
where
    Head: ProvideBundle<R, I>,
    R: Resources,
{
    type Ref<'resources> = Cons<Head::Ref<'resources>, Nil>
    where
        R: 'resources;

    fn provide(resources: &R) -> Self::Ref<'_> {
        let head = Head::provide(resources);
        Cons(head, Nil)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail, R, Index, TailIndex> ProvideBundle<R, Cons<Index, TailIndex>> for Cons<Head, Tail>
where
    Head: ProvideBundle<R, Index>,
    Tail: ProvideBundle<R, TailIndex> + Bundle,
    R: Resources,
{
    type Ref<'resources> = Cons<Head::Ref<'resources>, Tail::Ref<'resources>>
    where
        R: 'resources;

    fn provide(resources: &R) -> Self::Ref<'_> {
        let head = Head::provide(resources);
        let tail = Tail::provide(resources);
        Cons(head, tail)
    }
}

/// Trivial implementation for resources, which forwards implementation to the resource registry.
impl<T, R, I> ProvideBundleMut<R, I> for T
where
    T: Resource,
    R: ResourcesProvider<T, I>,
{
    type RefMut<'resources> = &'resources mut T
    where
        R: 'resources;

    fn provide_mut(resources: &mut R) -> Self::RefMut<'_> {
        resources.provide_mut()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head, R, I> ProvideBundleMut<R, I> for Cons<Head, Nil>
where
    Head: ProvideBundleMut<R, I>,
    R: Resources,
{
    type RefMut<'resources> = Cons<Head::RefMut<'resources>, Nil>
    where
        R: 'resources;

    fn provide_mut(resources: &mut R) -> Self::RefMut<'_> {
        let head = Head::provide_mut(resources);
        Cons(head, Nil)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail, R, Index, TailIndex> ProvideBundleMut<R, Cons<Index, TailIndex>>
    for Cons<Head, Tail>
where
    Head: ProvideBundleMut<R, Index>,
    Tail: ProvideBundleMut<R, TailIndex> + Bundle,
    R: Resources,
    for<'any> Head::RefMut<'any>: Dependency<&'any mut dyn Any>,
    for<'any> Tail::RefMut<'any>: Dependency<&'any mut dyn Any>,
{
    type RefMut<'resources> = Cons<Head::RefMut<'resources>, Tail::RefMut<'resources>>
    where
        R: 'resources;

    fn provide_mut(resources: &mut R) -> Self::RefMut<'_> {
        let iter = resources.iter_mut().map(AsAny::as_any_mut);
        dependency_from_iter(iter)
            .ok()
            .expect("all components of the bundle must be present in the registry")
    }
}
