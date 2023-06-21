use either::Either;
use hlist::{Cons, Nil};

use crate::{
    component::{
        registry::{Provider, Registry as Components},
        storage::{
            bundle::{
                GetBundleMut as StorageGetBundleMut, ProvideBundleMut as StorageProvideBundleMut,
            },
            Storage, TryStorage,
        },
        Component,
    },
    entity::Entity,
};

use super::{
    Bundle, GetBundle, GetBundleMut, NotRegisteredError, ProvideBundle, ProvideBundleMut,
    TryBundle, TryBundleError,
};

use self::impl_details::GetComponentsMut;

/// Trivial implementation for components, which forwards implementation to the component storage.
impl<T> Bundle for T
where
    T: Component,
{
    type Storages = T::Storage;

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
    type Storages = Cons<Head::Storages, Nil>;

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
    Tail: Bundle,
{
    type Storages = Cons<Head::Storages, Tail::Storages>;

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
    Tail: TryBundle,
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
    type Ref<'components> = &'components T;

    fn get<C>(components: &C, entity: Entity) -> Result<Option<Self::Ref<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let Some(storage) = components.get::<T>() else {
            return Err(NotRegisteredError::new::<Self>());
        };
        let component = Storage::get(storage, entity);
        Ok(component)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetBundle for Cons<Head, Nil>
where
    Head: GetBundle,
{
    type Ref<'components> = Cons<Head::Ref<'components>, Nil>;

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
    Tail: GetBundle,
{
    type Ref<'components> = Cons<Head::Ref<'components>, Tail::Ref<'components>>;

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
    type RefMut<'components> = &'components mut T;

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
        let component = Storage::get_mut(storage, entity);
        Ok(component)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetBundleMut for Cons<Head, Nil>
where
    Head: GetBundleMut,
{
    type RefMut<'components> = Cons<Head::RefMut<'components>, Nil>;

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
    Tail: GetBundleMut,
    Cons<Head::Storages, Tail::Storages>: StorageGetBundleMut,
    for<'any> <Cons<Head::Storages, Tail::Storages> as StorageGetBundleMut>::RefMut<'any>:
        GetComponentsMut<'any, Components = Cons<Head::RefMut<'any>, Tail::RefMut<'any>>>,
{
    type RefMut<'components> = Cons<Head::RefMut<'components>, Tail::RefMut<'components>>;

    fn get_mut<C>(
        components: &mut C,
        entity: Entity,
    ) -> Result<Option<Self::RefMut<'_>>, NotRegisteredError>
    where
        C: Components,
    {
        let _ = Self::is_attached(components, entity)?;
        let storages = <Self::Storages as StorageGetBundleMut>::get_mut(components)
            .expect("presence of all bundle components was checked earlier");
        let bundle = storages.get_components_mut(entity);
        Ok(bundle)
    }
}

impl<T, C, I> ProvideBundle<C, I> for T
where
    T: Component,
    C: Provider<T>,
{
    type Ref<'components> = &'components T
    where
        C: 'components;

    fn provide(components: &C, entity: Entity) -> Option<Self::Ref<'_>> {
        let storage = components.provide();
        Storage::get(storage, entity)
    }
}

impl<Head, C, I> ProvideBundle<C, I> for Cons<Head, Nil>
where
    Head: ProvideBundle<C, I>,
    C: Components,
{
    type Ref<'components> = Cons<Head::Ref<'components>, Nil>
    where
        C: 'components;

    fn provide(components: &C, entity: Entity) -> Option<Self::Ref<'_>> {
        let head = Head::provide(components, entity)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }
}

impl<Head, Tail, C, Index, TailIndex> ProvideBundle<C, Cons<Index, TailIndex>> for Cons<Head, Tail>
where
    Head: ProvideBundle<C, Index>,
    Tail: ProvideBundle<C, TailIndex> + Bundle,
    C: Components,
{
    type Ref<'components> = Cons<Head::Ref<'components>, Tail::Ref<'components>>
    where
        C: 'components;

    fn provide(components: &C, entity: Entity) -> Option<Self::Ref<'_>> {
        let head = Head::provide(components, entity)?;
        let tail = Tail::provide(components, entity)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }
}

impl<T, C, I> ProvideBundleMut<C, I> for T
where
    T: Component,
    C: Provider<T>,
{
    type RefMut<'components> = &'components mut T
    where
        C: 'components;

    fn provide_mut(components: &mut C, entity: Entity) -> Option<Self::RefMut<'_>> {
        let storage = components.provide_mut();
        Storage::get_mut(storage, entity)
    }
}

impl<Head, C, I> ProvideBundleMut<C, I> for Cons<Head, Nil>
where
    Head: ProvideBundleMut<C, I>,
    C: Components,
{
    type RefMut<'components> = Cons<Head::RefMut<'components>, Nil>
    where
        C: 'components;

    fn provide_mut(components: &mut C, entity: Entity) -> Option<Self::RefMut<'_>> {
        let head = Head::provide_mut(components, entity)?;
        let bundle = Cons(head, Nil);
        Some(bundle)
    }
}

impl<Head, Tail, C, Index, TailIndex> ProvideBundleMut<C, Cons<Index, TailIndex>> for Cons<Head, Tail>
where
    Head: ProvideBundleMut<C, Index>,
    Tail: ProvideBundleMut<C, TailIndex> + Bundle,
    C: Components,
    Cons<Head::Storages, Tail::Storages>: StorageProvideBundleMut<C, Cons<Index, TailIndex>>,
    for<'any> <Cons<Head::Storages, Tail::Storages> as StorageProvideBundleMut<C, Cons<Index, TailIndex>>>::RefMut<'any>:
        GetComponentsMut<'any, Components = Cons<Head::RefMut<'any>, Tail::RefMut<'any>>>,
{
    type RefMut<'components> = Cons<Head::RefMut<'components>, Tail::RefMut<'components>>
    where
        C: 'components;

    fn provide_mut(components: &mut C, entity: Entity) -> Option<Self::RefMut<'_>> {
        let storages = <Self::Storages as StorageProvideBundleMut<C, Cons<Index, TailIndex>>>::provide_mut(components);
        storages.get_components_mut(entity)
    }
}

mod impl_details {
    use hlist::{Cons, Nil};

    use crate::{component::storage::Storage, entity::Entity};

    pub trait GetComponentsMut<'components> {
        type Components: 'components;

        fn get_components_mut(self, entity: Entity) -> Option<Self::Components>;
    }

    impl<'components, T> GetComponentsMut<'components> for &'components mut T
    where
        T: Storage,
    {
        type Components = &'components mut T::Item;

        fn get_components_mut(self, entity: Entity) -> Option<Self::Components> {
            self.get_mut(entity)
        }
    }

    impl<'components, Head> GetComponentsMut<'components> for Cons<Head, Nil>
    where
        Head: GetComponentsMut<'components>,
    {
        type Components = Cons<Head::Components, Nil>;

        fn get_components_mut(self, entity: Entity) -> Option<Self::Components> {
            let Cons(head, nil) = self;
            let head = Head::get_components_mut(head, entity)?;
            let bundle = Cons(head, nil);
            Some(bundle)
        }
    }

    impl<'components, Head, Tail> GetComponentsMut<'components> for Cons<Head, Tail>
    where
        Head: GetComponentsMut<'components>,
        Tail: GetComponentsMut<'components>,
    {
        type Components = Cons<Head::Components, Tail::Components>;

        fn get_components_mut(self, entity: Entity) -> Option<Self::Components> {
            let Cons(head, tail) = self;
            let head = Head::get_components_mut(head, entity)?;
            let tail = Tail::get_components_mut(tail, entity)?;
            let bundle = Cons(head, tail);
            Some(bundle)
        }
    }
}
