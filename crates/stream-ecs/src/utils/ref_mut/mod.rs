use self::container::RefMutContainer;

mod container;
mod impls;

pub trait RefMut<'borrow> {
    type Container: RefMutContainer<'borrow, RefMut = Self>;
}

pub fn ref_mut<'borrow, R, I>(iter: I) -> Option<R>
where
    R: RefMut<'borrow>,
    I: IntoIterator<Item = &'borrow mut dyn core::any::Any>,
{
    let mut container = R::Container::default();
    for any in iter {
        container.insert_any(any);
    }
    container.into_ref_mut()
}
