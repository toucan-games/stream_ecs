use core::any::{type_name, Any, TypeId};

use derive_more::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[display(fmt = r#"no input of type "{type_name}" were provided"#)]
pub struct InputTypeMismatchError {
    type_name: &'static str,
    type_id: TypeId,
}

impl InputTypeMismatchError {
    pub fn new<T>() -> Self
    where
        T: ?Sized + Any,
    {
        let type_name = type_name::<T>();
        let type_id = TypeId::of::<T>();
        Self { type_name, type_id }
    }

    pub fn type_name(self) -> &'static str {
        let Self { type_name, .. } = self;
        type_name
    }

    pub fn type_id(self) -> TypeId {
        let Self { type_id, .. } = self;
        type_id
    }
}
