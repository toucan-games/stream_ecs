//! Utilities for error handling when operating with entity builder in ECS.

use crate::component::error::{NotRegisteredError, TryBundleError};

/// The result type which is returned when trying to build a new entity.
pub type TryEntityBuildResult<T, Err> = Result<T, TryEntityBuildError<Err>>;

/// The error type which is returned when trying to build a new entity.
#[derive(Debug, Clone, Copy)]
pub enum TryEntityBuildError<Err> {
    /// Component was not registered in the component registry.
    NotRegistered(NotRegisteredError),
    /// Entity registry failed to create new entity.
    Entities(Err),
}

impl<Err> From<NotRegisteredError> for TryEntityBuildError<Err> {
    fn from(error: NotRegisteredError) -> Self {
        Self::NotRegistered(error)
    }
}

impl<Err> core::fmt::Display for TryEntityBuildError<Err>
where
    Err: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotRegistered(error) => error.fmt(f),
            Self::Entities(error) => {
                write!(f, "Entity registry failed to create new entity: {error}")
            }
        }
    }
}

/// The result type which is returned when trying to build a new entity.
pub type TryBuildResult<T, EntitiesErr, StorageErr> =
    Result<T, TryBuildError<EntitiesErr, StorageErr>>;

/// The error type which is returned when trying to build a new entity.
#[derive(Debug, Clone, Copy)]
pub enum TryBuildError<EntitiesErr, StorageErr> {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Entity registry failed to create new entity.
    Entities(EntitiesErr),
    /// Component storage failed to attach a bundle to the entity.
    Storage(StorageErr),
}

impl<EntitiesErr, StorageErr> From<NotRegisteredError> for TryBuildError<EntitiesErr, StorageErr> {
    fn from(error: NotRegisteredError) -> Self {
        Self::NotRegistered(error)
    }
}

impl<EntitiesErr, StorageErr> From<TryBundleError<StorageErr>>
    for TryBuildError<EntitiesErr, StorageErr>
{
    fn from(error: TryBundleError<StorageErr>) -> Self {
        match error {
            TryBundleError::NotRegistered(error) => error.into(),
            TryBundleError::Storage(error) => Self::Storage(error),
        }
    }
}

impl<EntitiesErr, StorageErr> core::fmt::Display for TryBuildError<EntitiesErr, StorageErr>
where
    EntitiesErr: core::fmt::Display,
    StorageErr: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotRegistered(error) => error.fmt(f),
            Self::Entities(error) => {
                write!(f, "Entity registry failed to create new entity: {error}")
            }
            Self::Storage(error) => write!(f, "Storage failed to attach a component: {error}"),
        }
    }
}
