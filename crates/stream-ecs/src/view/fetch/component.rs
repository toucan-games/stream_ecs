use core::marker::PhantomData;

use crate::component::Component;

use super::Fetch;

/// Fetcher that fetches references of components.
pub struct FetchComponent<'a, C>(PhantomData<fn() -> &'a C>);

impl<'a, C> Fetch<'a> for FetchComponent<'a, C>
where
    C: Component,
{
    type Item = &'a C;
}
