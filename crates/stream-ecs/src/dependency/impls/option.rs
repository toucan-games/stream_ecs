use crate::dependency::{Container, Dependency};

#[derive(Default)]
pub struct OptionContainer<T>(T);

impl<T, Input> Dependency<Input> for Option<T>
where
    T: Dependency<Input>,
{
    type Container = OptionContainer<T::Container>;
}

impl<T, Input> Container<Input> for OptionContainer<T>
where
    T: Container<Input>,
{
    type Output = Option<T::Output>;

    fn insert(&mut self, input: Input) -> Result<(), Input> {
        let Self(container) = self;
        container.insert(input)
    }

    fn flush(self) -> Option<Self::Output> {
        let Self(container) = self;
        Some(container.flush())
    }
}
