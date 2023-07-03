pub trait TypeName {
    fn type_name(&self) -> &'static str;
}

impl<T> TypeName for T
where
    T: ?Sized,
{
    fn type_name(&self) -> &'static str {
        core::any::type_name::<T>()
    }
}
