use either::Either;
use hlist::{Cons, HList, Nil};

use crate::{
    component::{
        registry::Registry as Components,
        storage::{Storage, TryStorage},
        Component,
    },
    entity::Entity,
};

use super::{Bundle, GetBundle, GetBundleMut, NotRegisteredError, TryBundle, TryBundleError};

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> Bundle for T
where
    T: Component,
{
    fn attach<C>(
        components: &mut C,
        entity: Entity,
        component: Self,
    ) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.attach(entity, component);
        Ok(component)
    }

    fn remove<C>(components: &mut C, entity: Entity) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.remove(entity);
        Ok(component)
    }

    fn is_attached<C>(components: &C, entity: Entity) -> Result<bool, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let is_attached = storage.is_attached(entity);
        Ok(is_attached)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> Bundle for Cons<Head, Nil>
where
    Head: Bundle,
{
    fn attach<C>(
        components: &mut C,
        entity: Entity,
        bundle: Self,
    ) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let Cons(bundle, nil) = bundle;
        let Some(bundle) = Head::attach(components, entity, bundle)? else {
            return Ok(None);
        };
        let bundle = Cons(bundle, nil);
        Ok(Some(bundle))
    }

    fn remove<C>(components: &mut C, entity: Entity) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(bundle) = Head::remove(components, entity)? else {
            return Ok(None);
        };
        let bundle = Cons(bundle, Nil);
        Ok(Some(bundle))
    }

    fn is_attached<C>(components: &C, entity: Entity) -> Result<bool, NotRegisteredError>
    where
        C: Components,
    {
        Head::is_attached(components, entity)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> Bundle for Cons<Head, Tail>
where
    Head: Bundle,
    Tail: Bundle + HList,
{
    fn attach<C>(
        components: &mut C,
        entity: Entity,
        bundle: Self,
    ) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let _ = Self::is_attached(components, entity)?;
        let Cons(head, tail) = bundle;
        let Some(head) = Head::attach(components, entity, head)? else {
            return Ok(None);
        };
        let Some(tail) = Tail::attach(components, entity, tail)? else {
            return Ok(None);
        };
        let bundle = Cons(head, tail);
        Ok(Some(bundle))
    }

    fn remove<C>(components: &mut C, entity: Entity) -> Result<Option<Self>, NotRegisteredError>
    where
        C: Components,
    {
        let _ = Self::is_attached(components, entity)?;
        let Some(head) = Head::remove(components, entity)? else {
            return Ok(None);
        };
        let Some(tail) = Tail::remove(components, entity)? else {
            return Ok(None);
        };
        let bundle = Cons(head, tail);
        Ok(Some(bundle))
    }

    fn is_attached<C>(components: &C, entity: Entity) -> Result<bool, NotRegisteredError>
    where
        C: Components,
    {
        let head = Head::is_attached(components, entity)?;
        let tail = Tail::is_attached(components, entity)?;
        Ok(head && tail)
    }
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> TryBundle for T
where
    T: Component,
    T::Storage: TryStorage,
{
    type Err = <T::Storage as TryStorage>::Err;

    fn try_attach<C>(
        components: &mut C,
        entity: Entity,
        component: Self,
    ) -> Result<Option<Self>, TryBundleError<Self::Err>>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            let error = NotRegisteredError::new::<Self>();
            return Err(error.into());
        };
        let component = match storage.try_attach(entity, component) {
            Ok(component) => component,
            Err(err) => return Err(TryBundleError::Storage(err)),
        };
        Ok(component)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> TryBundle for Cons<Head, Nil>
where
    Head: TryBundle,
{
    type Err = Head::Err;

    fn try_attach<C>(
        components: &mut C,
        entity: Entity,
        bundle: Self,
    ) -> Result<Option<Self>, TryBundleError<Self::Err>>
    where
        C: Components,
    {
        let Cons(bundle, nil) = bundle;
        let Some(bundle) = Head::attach(components, entity, bundle)? else {
            return Ok(None);
        };
        let bundle = Cons(bundle, nil);
        Ok(Some(bundle))
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> TryBundle for Cons<Head, Tail>
where
    Head: TryBundle,
    Tail: TryBundle + HList,
{
    type Err = Either<Head::Err, Tail::Err>;

    fn try_attach<C>(
        components: &mut C,
        entity: Entity,
        bundle: Self,
    ) -> Result<Option<Self>, TryBundleError<Self::Err>>
    where
        C: Components,
    {
        let _ = Self::is_attached(components, entity)?;
        let Cons(head, tail) = bundle;
        let head = match Head::try_attach(components, entity, head) {
            Ok(Some(head)) => head,
            Ok(None) => return Ok(None),
            Err(error) => match error {
                TryBundleError::NotRegistered(error) => return Err(error.into()),
                TryBundleError::Storage(error) => {
                    let error = Either::Left(error);
                    return Err(TryBundleError::Storage(error));
                }
            },
        };
        let tail = match Tail::try_attach(components, entity, tail) {
            Ok(Some(head)) => head,
            Ok(None) => return Ok(None),
            Err(error) => match error {
                TryBundleError::NotRegistered(error) => return Err(error.into()),
                TryBundleError::Storage(error) => {
                    let error = Either::Right(error);
                    return Err(TryBundleError::Storage(error));
                }
            },
        };
        let bundle = Cons(head, tail);
        Ok(Some(bundle))
    }
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> GetBundle for T
where
    T: Component,
{
    type Ref<'a> = &'a Self
    where
        Self: 'a;

    fn get<C>(components: &C, entity: Entity) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.get(entity);
        Ok(component)
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

    fn get<C>(components: &C, entity: Entity) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(bundle) = Head::get(components, entity)? else {
            return Ok(None);
        };
        let bundle = Cons(bundle, Nil);
        Ok(Some(bundle))
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

    fn get<C>(components: &C, entity: Entity) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(head) = Head::get(components, entity)? else {
            return Ok(None);
        };
        let Some(tail) = Tail::get(components, entity)? else {
            return Ok(None);
        };
        let bundle = Cons(head, tail);
        Ok(Some(bundle))
    }
}

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> GetBundleMut for T
where
    T: Component,
{
    type RefMut<'a> = &'a mut Self
    where
        Self: 'a;

    fn get_mut<C>(
        components: &mut C,
        entity: Entity,
    ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get_mut::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = storage.get_mut(entity);
        Ok(component)
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

    fn get_mut<C>(
        components: &mut C,
        entity: Entity,
    ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(bundle) = Head::get_mut(components, entity)? else {
            return Ok(None);
        };
        let bundle = Cons(bundle, Nil);
        Ok(Some(bundle))
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> GetBundleMut for Cons<Head, Tail>
where
    Head: GetBundleMut,
    Tail: GetBundleMut + HList,
    for<'a> Tail::RefMut<'a>: HList,
{
    type RefMut<'a> = Cons<Head::RefMut<'a>, Tail::RefMut<'a>>
    where
        Self: 'a;

    fn get_mut<C>(
        _components: &mut C,
        _entity: Entity,
    ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        // TODO get multiple storages from the registry, then retrieve a mutable reference from each storage
        todo!()
    }
}
