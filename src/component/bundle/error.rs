use core::{any::TypeId, fmt::Display};

use crate::component::Component;

/// The error type which is returned when component type was not registered in the component registry.
#[derive(Debug, Clone, Copy)]
pub struct NotRegisteredError {
    type_name: &'static str,
    type_id: TypeId,
}

impl NotRegisteredError {
    /// Creates new error for the component type that was not registered in the component registry.
    pub fn new<T>() -> Self
    where
        T: Component,
    {
        Self {
            type_name: core::any::type_name::<T>(),
            type_id: TypeId::of::<T>(),
        }
    }

    /// Returns [`TypeId`] of component that was not registered in the component registry.
    pub fn type_id(self) -> TypeId {
        self.type_id
    }
}

impl Display for NotRegisteredError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type_name = self.type_name;
        write!(f, "Component of type {type_name} was not registered")
    }
}

/// The error type which is returned when trying to attach a bundle to the entity.
#[derive(Debug, Clone, Copy)]
pub enum TryBundleError<Err> {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Component storage failed to attach a bundle to the entity.
    Storage(Err),
}

impl<Err> From<NotRegisteredError> for TryBundleError<Err> {
    fn from(error: NotRegisteredError) -> Self {
        Self::NotRegistered(error)
    }
}

impl<Err> Display for TryBundleError<Err>
where
    Err: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotRegistered(error) => error.fmt(f),
            Self::Storage(error) => {
                write!(f, "Storage failed to attach a component: {error}")
            }
        }
    }
}
