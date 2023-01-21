//! Utilities for error handling when operating with components in ECS.

use core::{any::TypeId, fmt::Display};

use super::Component;

/// The result type which is returned when component type could be not registered in the component registry.
pub type NotRegisteredResult<T> = Result<T, NotRegisteredError>;

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
