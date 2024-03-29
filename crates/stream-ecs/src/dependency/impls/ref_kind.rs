use core::any::Any;

use ref_kind::{Many, RefKind};

use crate::dependency::{Container, Dependency};

use super::error::InputTypeMismatchError;

type Key<'kind> = Option<RefKind<'kind, dyn Any>>;

impl<'me, T> Dependency<Key<'me>> for &'me T
where
    T: Any,
{
    type Container = Option<&'me T>;
}

impl<'me, T> Container<Key<'me>> for Option<&'me T>
where
    T: Any,
{
    fn insert(&mut self, mut input: Key<'me>) -> Result<(), Key<'me>> {
        let Some(kind) = input.as_ref() else {
            return Err(input);
        };
        if !kind.get_ref().is::<T>() {
            return Err(input);
        }

        let Ok(shared) = input.try_move_ref(()) else {
            return Err(input);
        };
        let downcast = shared
            .downcast_ref()
            .expect("cast should be successful because type was checked earlier");
        *self = Some(downcast);
        Err(input)
    }

    type Output = &'me T;

    type Error = InputTypeMismatchError;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        self.ok_or_else(InputTypeMismatchError::new::<T>)
    }
}

impl<'me, T> Dependency<Key<'me>> for &'me mut T
where
    T: Any,
{
    type Container = Option<&'me mut T>;
}

impl<'me, T> Container<Key<'me>> for Option<&'me mut T>
where
    T: Any,
{
    fn insert(&mut self, mut input: Key<'me>) -> Result<(), Key<'me>> {
        let Some(kind) = input.as_ref() else {
            return Err(input);
        };
        if !kind.get_ref().is::<T>() {
            return Err(input);
        }

        let Ok(unique) = input.try_move_mut(()) else {
            return Err(input);
        };
        let downcast = unique
            .downcast_mut()
            .expect("cast should be successful because type was checked earlier");
        *self = Some(downcast);
        Err(input)
    }

    type Output = &'me mut T;

    type Error = InputTypeMismatchError;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        self.ok_or_else(InputTypeMismatchError::new::<T>)
    }
}
