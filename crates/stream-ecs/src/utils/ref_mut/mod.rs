mod container;
mod impls;

pub trait RefMut<'a> {
    type Container: self::container::RefMutContainer<'a, RefMut = Self>;
}

pub fn ref_mut<'a, R, I>(iter: I) -> Option<R>
where
    R: RefMut<'a>,
    I: IntoIterator<Item = &'a mut dyn core::any::Any>,
{
    use self::container::RefMutContainer;

    let mut container = R::Container::default();
    for any in iter {
        container.insert_any(any);
    }
    container.into_ref_mut()
}
