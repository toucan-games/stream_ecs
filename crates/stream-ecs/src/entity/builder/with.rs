use hlist::{Cons, Nil};

use crate::component::bundle::Bundle;

/// Create new value from the self with provided bundle.
pub trait With {
    /// Type of new value created from the self with provided bundle.
    type Output<B>
    where
        B: Bundle;

    /// Creates new value from the self with provided bundle.
    fn with<B>(self, bundle: B) -> Self::Output<B>
    where
        B: Bundle;
}

impl With for Nil {
    type Output<B>
        = Cons<B, Nil>
    where
        B: Bundle;

    fn with<B>(self, bundle: B) -> Self::Output<B>
    where
        B: Bundle,
    {
        Cons(bundle, self)
    }
}

impl<Head, Tail> With for Cons<Head, Tail>
where
    Head: Bundle,
    Tail: With,
{
    type Output<B>
        = Cons<Head, Tail::Output<B>>
    where
        B: Bundle;

    fn with<B>(self, bundle: B) -> Self::Output<B>
    where
        B: Bundle,
    {
        let Cons(head, tail) = self;
        let tail = tail.with(bundle);
        Cons(head, tail)
    }
}
