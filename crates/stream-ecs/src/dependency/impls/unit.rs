use core::convert::Infallible;

use crate::dependency::{Container, Dependency};

impl<Input> Dependency<Input> for () {
    type Container = Self;
}

impl<Input> Container<Input> for () {
    fn insert(&mut self, input: Input) -> Result<(), Input> {
        Err(input)
    }

    type Output = Self;

    type Error = Infallible;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        Ok(self)
    }
}
