//! Provides utilities for creation of dependencies from unknown count of inputs.

mod impls;

/// Type of dependency to be created from provided inputs.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Dependency<Input> {
    /// Type of container which stores provided inputs.
    type Container: Container<Input, Output = Self>;
}

/// Type of container which stores provided inputs into itself.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Container<Input>: Default {
    /// Inserts provided input into the container.
    ///
    /// # Errors
    ///
    /// This function returns an error (provided input object)
    /// if the container cannot insert provided input into itself.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn insert(&mut self, input: Input) -> Result<(), Input>;

    /// Type of dependency in which this container will be flushed.
    type Output;

    /// Type of error which can occur on container flushing.
    type Error;

    /// Flushes inserted inputs into dependency object.
    ///
    /// # Errors
    ///
    /// This function returns an error
    /// if it is not possible to create such dependency.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn flush(self) -> Result<Self::Output, Self::Error>;
}

/// Creates dependency from provided iterator of inputs.
pub fn dependency_from_iter<D, C, I>(iter: I) -> Result<D, C::Error>
where
    I: IntoIterator,
    D: Dependency<I::Item, Container = C>,
    C: Container<I::Item, Output = D>,
{
    let mut container = C::default();
    for item in iter {
        let _ = container.insert(item);
    }
    container.flush()
}
