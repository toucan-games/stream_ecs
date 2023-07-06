use core::any::{type_name, Any, TypeId};

use derive_more::{Display, From};

use crate::{
    component::{Component, ErasedComponent},
    entity::{Entity, ErasedEntity},
    utils::type_name::TypeName,
};

/// The error type which is returned when type of component or entity was mismatched
/// when trying to attach component to the entity with erased storage.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy, From)]
pub enum AttachError {
    /// Component type was mismatched.
    ComponentMismatch(ComponentMismatchError),
    /// Entity type was mismatched.
    EntityMismatch(EntityMismatchError),
}

/// The error type which is returned when type of component was mismatched.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy)]
#[display(
    fmt = r#"type of component in the storage was mismatched: \
        provided type is "{}", \
        but storage actually stores components of type "{}""#,
    "_0.provided_type_name",
    "_0.actual_type_name"
)]
pub struct ComponentMismatchError(TypeMismatchError);

impl ComponentMismatchError {
    /// Creates new error when type of component was mismatched.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new<Actual>(provided: &dyn ErasedComponent) -> Self
    where
        Actual: Component,
    {
        let provided = provided.as_any();
        let error = TypeMismatchError::new::<Actual>(provided);
        Self(error)
    }

    /// Returns type name of mismatched input component type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provided_type_name(self) -> &'static str {
        self.0.provided_type_name
    }

    /// Returns [`TypeId`] of mismatched input component type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provided_type_id(self) -> TypeId {
        self.0.provided_type_id
    }

    /// Returns type name of actual component type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn actual_type_name(self) -> &'static str {
        self.0.actual_type_name
    }

    /// Returns [`TypeId`] of actual entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn actual_type_id(self) -> TypeId {
        self.0.actual_type_id
    }
}

/// The error type which is returned when type of entity was mismatched.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy)]
#[display(
    fmt = r#"type of entity in the storage was mismatched: \
        provided type is "{}", \
        but storage actually tracks components with entities of type "{}""#,
    "_0.provided_type_name",
    "_0.actual_type_name"
)]
pub struct EntityMismatchError(TypeMismatchError);

impl EntityMismatchError {
    /// Creates new error when type of entity was mismatched.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new<Actual>(provided: &dyn ErasedEntity) -> Self
    where
        Actual: Entity,
    {
        let provided = provided.as_any();
        let error = TypeMismatchError::new::<Actual>(provided);
        Self(error)
    }

    /// Returns type name of mismatched input entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provided_type_name(self) -> &'static str {
        self.0.provided_type_name
    }

    /// Returns [`TypeId`] of mismatched input entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn provided_type_id(self) -> TypeId {
        self.0.provided_type_id
    }

    /// Returns type name of actual entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn actual_type_name(self) -> &'static str {
        self.0.actual_type_name
    }

    /// Returns [`TypeId`] of actual entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn actual_type_id(self) -> TypeId {
        self.0.actual_type_id
    }
}

#[derive(Debug, Clone, Copy)]
struct TypeMismatchError {
    provided_type_name: &'static str,
    provided_type_id: TypeId,
    actual_type_name: &'static str,
    actual_type_id: TypeId,
}

impl TypeMismatchError {
    fn new<Actual>(provided: &dyn Any) -> Self
    where
        Actual: Any,
    {
        let provided_type_id = provided.type_id();
        let actual_type_id = TypeId::of::<Actual>();
        debug_assert_ne!(provided_type_id, actual_type_id);

        let provided_type_name = provided.type_name();
        let actual_type_name = type_name::<Actual>();

        Self {
            provided_type_name,
            provided_type_id,
            actual_type_name,
            actual_type_id,
        }
    }
}
