use core::fmt::Display;

use crate::{
    component::bundle::{NotRegisteredError, TryBundleError},
    entity::registry::NotPresentError,
};

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

impl Display for EntityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotRegistered(error) => error.fmt(f),
            Self::NotPresent(error) => error.fmt(f),
        }
    }
}

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

impl<Err> Display for TryAttachError<Err>
where
    Err: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotRegistered(error) => error.fmt(f),
            Self::Storage(error) => write!(f, "storage failed to attach a component: {error}"),
            Self::NotPresent(error) => error.fmt(f),
        }
    }
}
