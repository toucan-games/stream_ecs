use core::any::Any;

use hlist::{Cons, Nil};

pub trait Container<Input>: Default {
    type Output;

    fn insert(&mut self, input: Input) -> Result<(), Input>;

    fn flush(self) -> Option<Self::Output>;
}

impl<'me, T> Container<&'me mut dyn Any> for Option<&'me mut T>
where
    T: Any,
{
    type Output = &'me mut T;

    fn insert(&mut self, input: &'me mut dyn Any) -> Result<(), &'me mut dyn Any> {
        if self.is_some() {
            return Err(input);
        }
        if !input.is::<T>() {
            return Err(input);
        }
        let ref_mut = input
            .downcast_mut()
            .expect("cast should be successful because type was checked earlier");
        *self = Some(ref_mut);
        Ok(())
    }

    fn flush(self) -> Option<Self::Output> {
        self
    }
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
