use core::any::Any;

use crate::dependency::{Container, Dependency};

use super::error::InputTypeMismatchError;

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
    fn insert(&mut self, input: &'me dyn Any) -> Result<(), &'me dyn Any> {
        if self.is_some() {
            return Err(input);
        }
        if !input.is::<T>() {
            return Err(input);
        }
        let downcast = input
            .downcast_ref()
            .expect("cast should be successful because type was checked earlier");
        *self = Some(downcast);
        Ok(())
    }

    type Output = &'me T;

    type Error = InputTypeMismatchError;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        self.ok_or_else(InputTypeMismatchError::new::<T>)
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
    fn insert(&mut self, input: &'me mut dyn Any) -> Result<(), &'me mut dyn Any> {
        if self.is_some() {
            return Err(input);
        }
        if !input.is::<T>() {
            return Err(input);
        }
        let downcast = input
            .downcast_mut()
            .expect("cast should be successful because type was checked earlier");
        *self = Some(downcast);
        Ok(())
    }

    type Output = &'me mut T;

    type Error = InputTypeMismatchError;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        self.ok_or_else(InputTypeMismatchError::new::<T>)
    }
}
