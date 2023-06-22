use hlist::{Cons, Nil};

use crate::utils::ref_mut::{Container, Dependency};

impl<Input> Dependency<Input> for Nil {
    type Container = Self;
}

impl<Input> Container<Input> for Nil {
    type Output = Self;

    fn insert(&mut self, input: Input) -> Result<(), Input> {
        Err(input)
    }

    fn flush(self) -> Option<Self::Output> {
        Some(self)
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
    type Output = Cons<Head::Output, Tail::Output>;

    fn insert(&mut self, input: Input) -> Result<(), Input> {
        let Cons(head, tail) = self;
        head.insert(input).or_else(|input| tail.insert(input))
    }

    fn flush(self) -> Option<Self::Output> {
        let Cons(head, tail) = self;
        let head = head.flush()?;
        let tail = tail.flush()?;
        let output = Cons(head, tail);
        Some(output)
    }
}
