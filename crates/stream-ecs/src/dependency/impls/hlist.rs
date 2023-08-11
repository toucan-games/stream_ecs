use core::convert::Infallible;

use either::Either;
use hlist::{Cons, Nil};

use crate::dependency::{Container, Dependency};

impl<Input> Dependency<Input> for Nil {
    type Container = Self;
}

impl<Input> Container<Input> for Nil {
    fn insert(&mut self, input: Input) -> Result<(), Input> {
        Err(input)
    }

    type Output = Self;

    type Error = Infallible;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        Ok(self)
    }
}

impl<Input, Head, Tail> Dependency<Input> for Cons<Head, Tail>
where
    Head: Dependency<Input>,
    Tail: Dependency<Input>,
{
    type Container = Cons<Head::Container, Tail::Container>;
}

impl<Input, Head, Tail> Container<Input> for Cons<Head, Tail>
where
    Head: Container<Input>,
    Tail: Container<Input>,
{
    fn insert(&mut self, input: Input) -> Result<(), Input> {
        let Cons(head, tail) = self;
        head.insert(input).or_else(|input| tail.insert(input))
    }

    type Output = Cons<Head::Output, Tail::Output>;

    type Error = Either<Head::Error, Tail::Error>;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        let Cons(head, tail) = self;
        let head = head.flush().map_err(Either::Left)?;
        let tail = tail.flush().map_err(Either::Right)?;
        let output = Cons(head, tail);
        Ok(output)
    }
}
