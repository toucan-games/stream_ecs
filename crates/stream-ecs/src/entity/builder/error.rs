use core::fmt::Display;

use crate::component::bundle::{NotRegisteredError, TryBundleError};

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

impl<Err> Display for TryEntityBuildError<Err>
where
    Err: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotRegistered(error) => error.fmt(f),
            Self::Entities(error) => {
                write!(f, "entity registry failed to create new entity: {error}")
            }
        }
    }
}

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

impl<EntitiesErr, StorageErr> Display for TryBuildError<EntitiesErr, StorageErr>
where
    EntitiesErr: Display,
    StorageErr: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotRegistered(error) => error.fmt(f),
            Self::Entities(error) => {
                write!(f, "entity registry failed to create new entity: {error}")
            }
            Self::Storage(error) => write!(f, "storage failed to attach a component: {error}"),
        }
    }
}
