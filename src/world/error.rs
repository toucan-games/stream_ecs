//! Utilities for error handling when operating with world in ECS.

use crate::{component::error::NotRegisteredError, entity::error::NotPresentError};

/// The result type which is returned when operating with entities in the ECS world.
pub type EntityResult<T> = Result<T, EntityError>;

/// The error type which is returned when operating with entities in the ECS world.
#[derive(Debug, Clone, Copy)]
pub enum EntityError {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Entity was not present in the world.
    NotPresent(NotPresentError),
}

impl From<NotRegisteredError> for EntityError {
    fn from(error: NotRegisteredError) -> Self {
        Self::NotRegistered(error)
    }
}

impl From<NotPresentError> for EntityError {
    fn from(error: NotPresentError) -> Self {
        Self::NotPresent(error)
    }
}

impl core::fmt::Display for EntityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EntityError::NotRegistered(error) => error.fmt(f),
            EntityError::NotPresent(error) => error.fmt(f),
        }
    }
}
