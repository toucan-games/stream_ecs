use core::any::{TypeId, type_name};

use derive_more::{Display, From};

use crate::component::Component;

/// The error type which is returned when component type was not registered in the component registry.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy)]
#[display(r#"component of type "{type_name}" was not registered"#)]
pub struct NotRegisteredError {
    type_name: &'static str,
    type_id: TypeId,
}

impl NotRegisteredError {
    /// Creates new error for the component type that was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new<T>() -> Self
    where
        T: Component,
    {
        Self {
            type_name: type_name::<T>(),
            type_id: TypeId::of::<T>(),
        }
    }

    /// Returns [`TypeId`] of component that was not registered in the component registry.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn type_id(self) -> TypeId {
        self.type_id
    }
}

/// The error type which is returned when trying to attach a bundle to the entity.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy, From)]
#[display(bound(Err: Display))]
pub enum TryBundleError<Err> {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Component storage failed to attach a bundle to the entity.
    #[from(ignore)]
    #[display("storage failed to attach a component: {_0}")]
    Storage(Err),
}
