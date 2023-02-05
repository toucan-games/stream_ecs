use core::any::{Any, TypeId};

use crate::component::Component;

/// The error type which is returned when type of component was mismatched
/// when trying to attach it to the entity with erased storage.
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
        write!(f, "type of component in the storage was mismatched: \
                   provided type is {provided}, but storage actually stores components of type {actual}")
    }
}

trait TypeName {
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
