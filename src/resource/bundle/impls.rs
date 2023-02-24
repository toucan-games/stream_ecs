use hlist::{
    tuple::{IntoHList, IntoTuple},
    Cons, HList, Nil,
};

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

macro_rules! bundle_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> Bundle for ($($types,)*)
        where
            $($types: Bundle,)*
        {
            fn insert<__R>(resources: &mut __R, bundle: Self) -> Option<Self>
            where
                __R: Resources,
            {
                let bundle = bundle.into_hlist();
                let bundle = Bundle::insert(resources, bundle)?;
                let bundle = bundle.into_tuple();
                Some(bundle)
            }

            fn remove<__R>(resources: &mut __R) -> Option<Self>
            where
                __R: Resources,
            {
                let bundle = <Self as IntoHList>::Output::remove(resources)?;
                let bundle = bundle.into_tuple();
                Some(bundle)
            }

            fn contains<__R>(resources: &__R) -> bool
            where
                __R: Resources,
            {
                <Self as IntoHList>::Output::contains(resources)
            }
        }
    };
}

// `Bundle` is implemented for tuples of size 12 and less
bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J);
bundle_for_tuple!(A, B, C, D, E, F, G, H, I);
bundle_for_tuple!(A, B, C, D, E, F, G, H);
bundle_for_tuple!(A, B, C, D, E, F, G);
bundle_for_tuple!(A, B, C, D, E, F);
bundle_for_tuple!(A, B, C, D, E);
bundle_for_tuple!(A, B, C, D);
bundle_for_tuple!(A, B, C);
bundle_for_tuple!(A, B);
bundle_for_tuple!(A);

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

macro_rules! try_bundle_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> TryBundle for ($($types,)*)
        where
            $($types: TryBundle,)*
        {
            fn try_insert<__R>(resources: &mut __R, bundle: Self) -> Result<Option<Self>, __R::Err>
            where
                __R: TryResources,
            {
                let bundle = bundle.into_hlist();
                let Some(bundle) = TryBundle::try_insert(resources, bundle)? else {
                    return Ok(None);
                };
                let bundle = bundle.into_tuple();
                Ok(Some(bundle))
            }
        }
    };
}

// `TryBundle` is implemented for tuples of size 12 and less
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H, I);
try_bundle_for_tuple!(A, B, C, D, E, F, G, H);
try_bundle_for_tuple!(A, B, C, D, E, F, G);
try_bundle_for_tuple!(A, B, C, D, E, F);
try_bundle_for_tuple!(A, B, C, D, E);
try_bundle_for_tuple!(A, B, C, D);
try_bundle_for_tuple!(A, B, C);
try_bundle_for_tuple!(A, B);
try_bundle_for_tuple!(A);

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

macro_rules! get_bundle_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> GetBundle for ($($types,)*)
        where
            $($types: GetBundle,)*
        {
            type Ref<'a> = ($($types::Ref<'a>,)*)
            where
                Self: 'a;

            fn get<__R>(resources: &__R) -> Option<Self::Ref<'_>>
            where
                __R: Resources,
            {
                let bundle = <Self as IntoHList>::Output::get(resources)?;
                let bundle = bundle.into_tuple();
                Some(bundle)
            }
        }
    };
}

// `GetBundle` is implemented for tuples of size 12 and less
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H, I);
get_bundle_for_tuple!(A, B, C, D, E, F, G, H);
get_bundle_for_tuple!(A, B, C, D, E, F, G);
get_bundle_for_tuple!(A, B, C, D, E, F);
get_bundle_for_tuple!(A, B, C, D, E);
get_bundle_for_tuple!(A, B, C, D);
get_bundle_for_tuple!(A, B, C);
get_bundle_for_tuple!(A, B);
get_bundle_for_tuple!(A);

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

impl<Head, Tail> GetBundleMut for Cons<Head, Tail>
where
    Head: GetBundleMut,
    Tail: GetBundleMut + HList,
    for<'a> Tail::RefMut<'a>: HList,
{
    type RefMut<'a> = Cons<Head::RefMut<'a>, Tail::RefMut<'a>>
    where
        Self: 'a;

    fn get_mut<R>(_resources: &mut R) -> Option<Self::RefMut<'_>>
    where
        R: Resources,
    {
        // TODO get multiple resources from the registry
        todo!()
    }
}

macro_rules! get_bundle_mut_for_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> GetBundleMut for ($($types,)*)
        where
            $($types: GetBundleMut,)*
        {
            type RefMut<'a> = ($($types::RefMut<'a>,)*)
            where
                Self: 'a;

            fn get_mut<__R>(resources: &mut __R) -> Option<Self::RefMut<'_>>
            where
                __R: Resources,
            {
                let bundle = <Self as IntoHList>::Output::get_mut(resources)?;
                let bundle = bundle.into_tuple();
                Some(bundle)
            }
        }
    };
}

// `GetBundleMut` is implemented for tuples of size 12 and less
get_bundle_mut_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
get_bundle_mut_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
get_bundle_mut_for_tuple!(A, B, C, D, E, F, G, H, I, J);
get_bundle_mut_for_tuple!(A, B, C, D, E, F, G, H, I);
get_bundle_mut_for_tuple!(A, B, C, D, E, F, G, H);
get_bundle_mut_for_tuple!(A, B, C, D, E, F, G);
get_bundle_mut_for_tuple!(A, B, C, D, E, F);
get_bundle_mut_for_tuple!(A, B, C, D, E);
get_bundle_mut_for_tuple!(A, B, C, D);
get_bundle_mut_for_tuple!(A, B, C);
get_bundle_mut_for_tuple!(A, B);
get_bundle_mut_for_tuple!(A);
