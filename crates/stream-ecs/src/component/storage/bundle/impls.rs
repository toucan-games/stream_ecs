use core::any::Any;

use as_any::AsAny;
use hlist::{Cons, Nil};

use crate::{
    component::{
        registry::{
            Provider as ComponentsProvider, Registry as Components, RegistryMut as ComponentsMut,
            TryRegistryMut as TryComponentsMut,
        },
        storage::Storage,
    },
    dependency::{dependency_from_iter, Dependency},
};

use super::{
    Bundle, GetBundle, GetBundleMut, GetItems, GetItemsMut, ProvideBundle, ProvideBundleMut,
    TryBundle,
};

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> Bundle for T
where
    T: Storage,
{
    type Items = T::Item;
    type Entity = T::Entity;

    type With<C> = C::With<T::Item>
    where
        C: Components;

    fn with<C>(components: C, bundle: Self) -> Self::With<C>
    where
        C: Components,
    {
        components.with(bundle)
    }

    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: ComponentsMut,
    {
        components.register::<T::Item>(bundle)
    }

    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: ComponentsMut,
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
    type Items = Cons<Head::Items, Nil>;
    type Entity = Head::Entity;

    type With<C> = Cons<Head::With<C>, Nil>
    where
        C: Components;

    fn with<C>(components: C, bundle: Self) -> Self::With<C>
    where
        C: Components,
    {
        let Cons(head, nil) = bundle;
        let head = Head::with(components, head);
        Cons(head, nil)
    }

    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: ComponentsMut,
    {
        let Cons(head, nil) = bundle;
        let head = Head::register(components, head)?;
        let bundle = Cons(head, nil);
        Some(bundle)
    }

    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: ComponentsMut,
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
    Tail: Bundle<Entity = Head::Entity>,
{
    type Items = Cons<Head::Items, Tail::Items>;
    type Entity = Head::Entity;

    type With<C> = Cons<Head::With<C>, Tail>
    where
        C: Components;

    fn with<C>(components: C, bundle: Self) -> Self::With<C>
    where
        C: Components,
    {
        let Cons(head, tail) = bundle;
        let head = Head::with(components, head);
        Cons(head, tail)
    }

    fn register<C>(components: &mut C, bundle: Self) -> Option<Self>
    where
        C: ComponentsMut,
    {
        let Cons(head, tail) = bundle;
        let head = Head::register(components, head)?;
        let tail = Tail::register(components, tail)?;
        let bundle = Cons(head, tail);
        Some(bundle)
    }

    fn unregister<C>(components: &mut C) -> Option<Self>
    where
        C: ComponentsMut,
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
        C: TryComponentsMut,
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
        C: TryComponentsMut,
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
    Tail: TryBundle<Entity = Head::Entity>,
{
    fn try_register<C>(components: &mut C, bundle: Self) -> Result<Option<Self>, C::Err>
    where
        C: TryComponentsMut,
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
    type Ref<'components> = &'components T;

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
    type Ref<'components> = Cons<Head::Ref<'components>, Nil>;

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
    Tail: GetBundle<Entity = Head::Entity>,
{
    type Ref<'components> = Cons<Head::Ref<'components>, Tail::Ref<'components>>;

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
    type RefMut<'components> = &'components mut T;

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
    type RefMut<'components> = Cons<Head::RefMut<'components>, Nil>;

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
    Tail: GetBundleMut<Entity = Head::Entity>,
    for<'any> Head::RefMut<'any>: Dependency<&'any mut dyn Any>,
    for<'any> Tail::RefMut<'any>: Dependency<&'any mut dyn Any>,
{
    type RefMut<'components> = Cons<Head::RefMut<'components>, Tail::RefMut<'components>>;

    fn get_mut<C>(components: &mut C) -> Option<Self::RefMut<'_>>
    where
        C: Components,
    {
        let iter = components.iter_mut().map(AsAny::as_any_mut);
        dependency_from_iter(iter)
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T, C, I> ProvideBundle<C, I> for T
where
    T: Storage,
    C: ComponentsProvider<T::Item, I>,
{
    type Ref<'components> = &'components T
    where
        C: 'components;

    fn provide(components: &C) -> Self::Ref<'_> {
        components.provide()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head, C, I> ProvideBundle<C, I> for Cons<Head, Nil>
where
    Head: ProvideBundle<C, I>,
    C: Components,
{
    type Ref<'components> = Cons<Head::Ref<'components>, Nil>
    where
        C: 'components;

    fn provide(components: &C) -> Self::Ref<'_> {
        let head = Head::provide(components);
        Cons(head, Nil)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail, C, Index, TailIndex> ProvideBundle<C, Cons<Index, TailIndex>> for Cons<Head, Tail>
where
    Head: ProvideBundle<C, Index>,
    Tail: ProvideBundle<C, TailIndex> + Bundle<Entity = Head::Entity>,
    C: Components,
{
    type Ref<'components> = Cons<Head::Ref<'components>, Tail::Ref<'components>>
    where
        C: 'components;

    fn provide(components: &C) -> Self::Ref<'_> {
        let head = Head::provide(components);
        let tail = Tail::provide(components);
        Cons(head, tail)
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T, C, I> ProvideBundleMut<C, I> for T
where
    T: Storage,
    C: ComponentsProvider<T::Item, I>,
{
    type RefMut<'components> = &'components mut T
    where
        C: 'components;

    fn provide_mut(components: &mut C) -> Self::RefMut<'_> {
        components.provide_mut()
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head, C, I> ProvideBundleMut<C, I> for Cons<Head, Nil>
where
    Head: ProvideBundleMut<C, I>,
    C: Components,
{
    type RefMut<'components> = Cons<Head::RefMut<'components>, Nil>
    where
        C: 'components;

    fn provide_mut(components: &mut C) -> Self::RefMut<'_> {
        let head = Head::provide_mut(components);
        Cons(head, Nil)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail, C, Index, TailIndex> ProvideBundleMut<C, Cons<Index, TailIndex>>
    for Cons<Head, Tail>
where
    Head: ProvideBundleMut<C, Index>,
    Tail: ProvideBundleMut<C, TailIndex> + Bundle<Entity = Head::Entity>,
    C: Components,
    for<'any> Head::RefMut<'any>: Dependency<&'any mut dyn Any>,
    for<'any> Tail::RefMut<'any>: Dependency<&'any mut dyn Any>,
{
    type RefMut<'components> = Cons<Head::RefMut<'components>, Tail::RefMut<'components>>
    where
        C: 'components;

    fn provide_mut(components: &mut C) -> Self::RefMut<'_> {
        let iter = components.iter_mut().map(AsAny::as_any_mut);
        dependency_from_iter(iter)
            .expect("all components of the bundle must be present in the registry")
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> GetItems for T
where
    T: Storage,
{
    type ItemsRef<'me> = &'me T::Item
    where
        Self: 'me;

    fn items(&self, entity: Self::Entity) -> Option<Self::ItemsRef<'_>> {
        self.get(entity)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetItems for Cons<Head, Nil>
where
    Head: GetItems,
{
    type ItemsRef<'me> = Cons<Head::ItemsRef<'me>, Nil>
    where
        Self: 'me;

    fn items(&self, entity: Self::Entity) -> Option<Self::ItemsRef<'_>> {
        let Cons(head, _) = self;
        let head = head.items(entity)?;
        let items = Cons(head, Nil);
        Some(items)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> GetItems for Cons<Head, Tail>
where
    Head: GetItems,
    Tail: GetItems<Entity = Head::Entity>,
{
    type ItemsRef<'me> = Cons<Head::ItemsRef<'me>, Tail::ItemsRef<'me>>
    where
        Self: 'me;

    fn items(&self, entity: Self::Entity) -> Option<Self::ItemsRef<'_>> {
        let Cons(head, tail) = self;
        let head = head.items(entity)?;
        let tail = tail.items(entity)?;
        let items = Cons(head, tail);
        Some(items)
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> GetItemsMut for T
where
    T: Storage,
{
    type ItemsRefMut<'me> = &'me mut T::Item
    where
        Self: 'me;

    fn items_mut(&mut self, entity: Self::Entity) -> Option<Self::ItemsRefMut<'_>> {
        self.get_mut(entity)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetItemsMut for Cons<Head, Nil>
where
    Head: GetItemsMut,
{
    type ItemsRefMut<'me> = Cons<Head::ItemsRefMut<'me>, Nil>
    where
        Self: 'me;

    fn items_mut(&mut self, entity: Self::Entity) -> Option<Self::ItemsRefMut<'_>> {
        let Cons(head, _) = self;
        let head = head.items_mut(entity)?;
        let items = Cons(head, Nil);
        Some(items)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail> GetItemsMut for Cons<Head, Tail>
where
    Head: GetItemsMut,
    Tail: GetItemsMut<Entity = Head::Entity>,
{
    type ItemsRefMut<'me> = Cons<Head::ItemsRefMut<'me>, Tail::ItemsRefMut<'me>>
    where
        Self: 'me;

    fn items_mut(&mut self, entity: Self::Entity) -> Option<Self::ItemsRefMut<'_>> {
        let Cons(head, tail) = self;
        let head = head.items_mut(entity)?;
        let tail = tail.items_mut(entity)?;
        let items = Cons(head, tail);
        Some(items)
    }
}
