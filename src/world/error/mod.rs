//! Utilities for error handling when operating with world in ECS.

use crate::{
    component::error::{NotRegisteredError, TryBundleError},
    entity::error::NotPresentError,
};

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

/// The result type which is returned when trying to attach a bundle to the entity in the world.
pub type TryAttachResult<T, Err> = Result<T, TryAttachError<Err>>;

/// The error type which is returned when trying to attach a bundle to the entity in the world.
#[derive(Debug, Clone, Copy)]
pub enum TryAttachError<Err> {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Component storage failed to attach a bundle to the entity.
    Storage(Err),
    /// Entity was not present in the world.
    NotPresent(NotPresentError),
}

impl<Err> From<NotRegisteredError> for TryAttachError<Err> {
    fn from(error: NotRegisteredError) -> Self {
        Self::NotRegistered(error)
    }
}

impl<Err> From<NotPresentError> for TryAttachError<Err> {
    fn from(error: NotPresentError) -> Self {
        Self::NotPresent(error)
    }
}

impl<Err> From<TryBundleError<Err>> for TryAttachError<Err> {
    fn from(error: TryBundleError<Err>) -> Self {
        match error {
            TryBundleError::NotRegistered(error) => error.into(),
            TryBundleError::Storage(error) => Self::Storage(error),
        }
    }
}

impl<Err> core::fmt::Display for TryAttachError<Err>
where
    Err: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TryAttachError::NotRegistered(error) => error.fmt(f),
            TryAttachError::Storage(error) => error.fmt(f),
            TryAttachError::NotPresent(error) => error.fmt(f),
        }
    }
}
