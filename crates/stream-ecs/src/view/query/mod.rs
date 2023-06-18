use super::fetch::Fetch;

mod impls;

/// Type of query to be queried from components by view.
pub trait Query: sealed::Sealed {
    /// Type of result yielded by the query.
    type Item<'a>;

    #[doc(hidden)]
    type Fetch<'a>: Fetch<'a, Item = Self::Item<'a>>;
}

mod sealed {
    use hlist::{Cons, Nil};

    use crate::{component::Component, entity::Entity};

    pub trait Sealed {}

    impl Sealed for Entity {}

    impl<C> Sealed for &C where C: Component {}

    impl<T> Sealed for Option<T> where T: Sealed {}

    impl<Head> Sealed for Cons<Head, Nil> where Head: Sealed {}

    impl<Head, Tail> Sealed for Cons<Head, Tail>
    where
        Head: Sealed,
        Tail: Sealed,
    {
    }
}
