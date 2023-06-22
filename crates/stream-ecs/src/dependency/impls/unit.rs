use crate::dependency::{Container, Dependency};

impl<Input> Dependency<Input> for () {
    type Container = Self;
}

impl<Input> Container<Input> for () {
    type Output = Self;

    fn insert(&mut self, input: Input) -> Result<(), Input> {
        Err(input)
    }

    fn flush(self) -> Option<Self::Output> {
        Some(self)
    }
}
