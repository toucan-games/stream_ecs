use core::marker::PhantomData;

use crate::component::Component;

use super::Fetch;

/// Fetcher that fetches references of components.
pub struct FetchComponentMut<'a, C>(PhantomData<fn() -> &'a mut C>);

impl<'a, C> Fetch<'a> for FetchComponentMut<'a, C>
where
    C: Component,
{
    type Item = &'a mut C;
}
