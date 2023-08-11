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
    fn insert(&mut self, input: Input) -> Result<(), Input> {
        let Self(container) = self;
        container.insert(input)
    }

    type Output = Option<T::Output>;

    type Error = T::Error;

    fn flush(self) -> Result<Self::Output, Self::Error> {
        let Self(container) = self;
        let dependency = container.flush()?;
        Ok(Some(dependency))
    }
}
