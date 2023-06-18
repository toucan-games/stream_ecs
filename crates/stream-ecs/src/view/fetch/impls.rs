use core::marker::PhantomData;

use hlist::{Cons, Nil};

use crate::{component::Component, entity::Entity};

use super::Fetch;

pub struct FetchEntity;

impl Fetch<'_> for FetchEntity {
    type Item = Entity;
}

pub struct FetchComponent<'a, C>(PhantomData<fn() -> &'a C>);

impl<'a, C> Fetch<'a> for FetchComponent<'a, C>
where
    C: Component,
{
    type Item = &'a C;
}

pub struct FetchOption<'a, T>(PhantomData<fn() -> &'a T>);

impl<'a, T> Fetch<'a> for FetchOption<'a, T>
where
    T: Fetch<'a>,
{
    type Item = Option<T::Item>;
}

impl<'a, Head> Fetch<'a> for Cons<Head, Nil>
where
    Head: Fetch<'a>,
{
    type Item = Cons<Head::Item, Nil>;
}

impl<'a, Head, Tail> Fetch<'a> for Cons<Head, Tail>
where
    Head: Fetch<'a>,
    Tail: Fetch<'a>,
{
    type Item = Cons<Head::Item, Tail::Item>;
}
