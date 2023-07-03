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
    /// Type of dependency in which this container will be flushed.
    type Output;

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

    /// Flushes inserted inputs into dependency object.
    /// Returns [`None`] if it is not possible to create such dependency.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    fn flush(self) -> Option<Self::Output>;
}

/// Creates dependency from provided iterator of inputs.
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
