//! Utilities for error handling when operating with components in ECS.

use core::any::{Any, TypeId};

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

impl core::fmt::Display for NotRegisteredError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type_name = self.type_name;
        write!(f, "Component of type {type_name} was not registered")
    }
}

/// The result type which is returned when type of component could be mismatched
/// when trying to attach it to the entity with [erased storage](super::storage::ErasedStorage).
pub type TypeMismatchResult<T> = Result<T, TypeMismatchError>;

/// The error type which is returned when type of component was mismatched
/// when trying to attach it to the entity with [erased storage](super::storage::ErasedStorage).
#[derive(Debug, Clone, Copy)]
pub struct TypeMismatchError {
    provided_type_name: &'static str,
    provided_type_id: TypeId,
    actual_type_name: &'static str,
    actual_type_id: TypeId,
}

impl TypeMismatchError {
    /// Creates new error for the component type that was mismatched.
    pub fn new<Actual>(provided: &dyn Any) -> Self
    where
        Actual: Component,
    {
        let provided_type_id = provided.type_id();
        let actual_type_id = TypeId::of::<Actual>();
        debug_assert_ne!(provided_type_id, actual_type_id);

        let provided_type_name = provided.type_name();
        let actual_type_name = core::any::type_name::<Actual>();

        Self {
            provided_type_name,
            provided_type_id,
            actual_type_name,
            actual_type_id,
        }
    }

    /// Returns [`TypeId`] of component that was provided by the caller.
    pub fn provided_type_id(self) -> TypeId {
        self.provided_type_id
    }

    /// Returns [`TypeId`] of component that was actually stored in the storage.
    pub fn actual_type_id(self) -> TypeId {
        self.actual_type_id
    }
}

impl core::fmt::Display for TypeMismatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let provided = self.provided_type_name;
        let actual = self.actual_type_name;
        write!(f, "Type of component in the storage was mismatched: provided type {provided}, but storage actually stores components of type {actual}")
    }
}

trait AnyTypeName: Any {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyTypeName for T
where
    T: Any + ?Sized,
{
    fn type_name(&self) -> &'static str {
        core::any::type_name::<T>()
    }
}
