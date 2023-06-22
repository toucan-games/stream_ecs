use core::any::Any;

use crate::dependency::{Container, Dependency};

impl<'me, T> Dependency<&'me dyn Any> for &'me T
where
    T: Any,
{
    type Container = Option<&'me T>;
}

impl<'me, T> Container<&'me dyn Any> for Option<&'me T>
where
    T: Any,
{
    type Output = &'me T;

    fn insert(&mut self, input: &'me dyn Any) -> Result<(), &'me dyn Any> {
        if self.is_some() {
            return Err(input);
        }
        if !input.is::<T>() {
            return Err(input);
        }
        let ref_mut = input
            .downcast_ref()
            .expect("cast should be successful because type was checked earlier");
        *self = Some(ref_mut);
        Ok(())
    }

    fn flush(self) -> Option<Self::Output> {
        self
    }
}

impl<'me, T> Dependency<&'me mut dyn Any> for &'me mut T
where
    T: Any,
{
    type Container = Option<&'me mut T>;
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
