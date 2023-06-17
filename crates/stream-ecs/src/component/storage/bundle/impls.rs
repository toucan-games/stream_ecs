use hlist::{Cons, Nil};

use crate::{
    component::{
        registry::{
            Provider as ComponentsProvider, Registry as Components, RegistryMut as ComponentsMut,
            TryRegistryMut as TryComponentsMut,
        },
        storage::Storage,
    },
    entity::Entity,
    ref_mut::{RefMut, RefMutContainer},
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
    Tail: Bundle,
{
    type Items = Cons<Head::Items, Tail::Items>;

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
    Tail: TryBundle,
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
    Tail: GetBundle,
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
    Tail: GetBundleMut,
    for<'a> Head::RefMut<'a>: RefMut<'a>,
    for<'a> Tail::RefMut<'a>: RefMut<'a>,
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

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T, C, I> ProvideBundle<C, I> for T
where
    T: Storage,
    C: ComponentsProvider<T::Item, I>,
{
    type Ref<'a> = &'a T
    where
        C: 'a;

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
    type Ref<'a> = Cons<Head::Ref<'a>, Nil>
    where
        C: 'a;

    fn provide(components: &C) -> Self::Ref<'_> {
        let head = Head::provide(components);
        Cons(head, Nil)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail, C, Index, TailIndex> ProvideBundle<C, (Index, TailIndex)> for Cons<Head, Tail>
where
    Head: ProvideBundle<C, Index>,
    Tail: ProvideBundle<C, TailIndex> + Bundle,
    C: Components,
{
    type Ref<'a> = Cons<Head::Ref<'a>, Tail::Ref<'a>>
    where
        C: 'a;

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
    type RefMut<'a> = &'a mut T
    where
        C: 'a;

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
    type RefMut<'a> = Cons<Head::RefMut<'a>, Nil>
    where
        C: 'a;

    fn provide_mut(components: &mut C) -> Self::RefMut<'_> {
        let head = Head::provide_mut(components);
        Cons(head, Nil)
    }
}

/// More complex implementation for heterogenous list with more than one element.
impl<Head, Tail, C, Index, TailIndex> ProvideBundleMut<C, (Index, TailIndex)> for Cons<Head, Tail>
where
    Head: ProvideBundleMut<C, Index>,
    Tail: ProvideBundleMut<C, TailIndex> + Bundle,
    C: Components,
    for<'a> Head::RefMut<'a>: RefMut<'a>,
    for<'a> Tail::RefMut<'a>: RefMut<'a>,
{
    type RefMut<'a> = Cons<Head::RefMut<'a>, Tail::RefMut<'a>>
    where
        C: 'a;

    fn provide_mut(components: &mut C) -> Self::RefMut<'_> {
        type Container<'a, T> = <T as RefMut<'a>>::Container;

        let mut container: Container<Self::RefMut<'_>> = Default::default();
        for storage in components.iter_mut() {
            let any = storage.as_any_mut();
            container.insert_any(any);
        }
        container
            .into_ref_mut()
            .expect("all components of the bundle must be present in the registry")
    }
}

/// Trivial implementation for storages, which forwards implementation to the component registry.
impl<T> GetItems for T
where
    T: Storage,
{
    type ItemsRef<'a> = &'a T::Item
    where
        Self: 'a;

    fn items(&self, entity: Entity) -> Option<Self::ItemsRef<'_>> {
        self.get(entity)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetItems for Cons<Head, Nil>
where
    Head: GetItems,
{
    type ItemsRef<'a> = Cons<Head::ItemsRef<'a>, Nil>
    where
        Self: 'a;

    fn items(&self, entity: Entity) -> Option<Self::ItemsRef<'_>> {
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
    Tail: GetItems,
{
    type ItemsRef<'a> = Cons<Head::ItemsRef<'a>, Tail::ItemsRef<'a>>
    where
        Self: 'a;

    fn items(&self, entity: Entity) -> Option<Self::ItemsRef<'_>> {
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
    type ItemsRefMut<'a> = &'a mut T::Item
    where
        Self: 'a;

    fn items_mut(&mut self, entity: Entity) -> Option<Self::ItemsRefMut<'_>> {
        self.get_mut(entity)
    }
}

/// More complex implementation for heterogenous list with single element.
impl<Head> GetItemsMut for Cons<Head, Nil>
where
    Head: GetItemsMut,
{
    type ItemsRefMut<'a> = Cons<Head::ItemsRefMut<'a>, Nil>
    where
        Self: 'a;

    fn items_mut(&mut self, entity: Entity) -> Option<Self::ItemsRefMut<'_>> {
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
    Tail: GetItemsMut,
{
    type ItemsRefMut<'a> = Cons<Head::ItemsRefMut<'a>, Tail::ItemsRefMut<'a>>
    where
        Self: 'a;

    fn items_mut(&mut self, entity: Entity) -> Option<Self::ItemsRefMut<'_>> {
        let Cons(head, tail) = self;
        let head = head.items_mut(entity)?;
        let tail = tail.items_mut(entity)?;
        let items = Cons(head, tail);
        Some(items)
    }
}
