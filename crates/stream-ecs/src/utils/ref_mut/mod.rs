use self::container::Container;

mod container;
mod impls;

pub trait Dependency<Input> {
    type Container: Container<Input, Output = Self>;
}

pub fn dependency_from_iter<D, I>(iter: I) -> Option<D>
where
    I: IntoIterator,
    D: Dependency<I::Item>,
{
    let mut container = D::Container::default();
    for item in iter {
        let _ = container.insert(item);
    }
    container.flush()
}
