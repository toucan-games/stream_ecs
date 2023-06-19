use core::marker::PhantomData;

use super::Fetch;

/// Fetcher that fetches optional data from the underlying fetcher.
pub struct FetchOption<'a, T>(PhantomData<fn() -> &'a T>)
where
    T: Fetch<'a>;

impl<'a, T> Fetch<'a> for FetchOption<'a, T>
where
    T: Fetch<'a>,
{
    type Item = Option<T::Item>;
}
