use hlist::{Cons, Nil};

use crate::{
    component::Component,
    entity::Entity,
    view::fetch::{FetchComponent, FetchComponentMut, FetchEntity, FetchOption},
};

use super::Query;

impl Query for () {
    type Item<'a> = ();
    type Fetch<'a> = ();
}

impl Query for Entity {
    type Item<'a> = Entity;
    type Fetch<'a> = FetchEntity;
}

impl<C> Query for &C
where
    C: Component,
{
    type Item<'a> = &'a C;
    type Fetch<'a> = FetchComponent<'a, C>;
}

impl<C> Query for &mut C
where
    C: Component,
{
    type Item<'a> = &'a mut C;
    type Fetch<'a> = FetchComponentMut<'a, C>;
}

impl<T> Query for Option<T>
where
    T: Query,
{
    type Item<'a> = Option<T::Item<'a>>;
    type Fetch<'a> = FetchOption<T::Fetch<'a>>;
}

impl<Head> Query for Cons<Head, Nil>
where
    Head: Query,
{
    type Item<'a> = Cons<Head::Item<'a>, Nil>;
    type Fetch<'a> = Cons<Head::Fetch<'a>, Nil>;
}

impl<Head, Tail> Query for Cons<Head, Tail>
where
    Head: Query,
    Tail: Query,
{
    type Item<'a> = Cons<Head::Item<'a>, Tail::Item<'a>>;
    type Fetch<'a> = Cons<Head::Fetch<'a>, Tail::Fetch<'a>>;
}
