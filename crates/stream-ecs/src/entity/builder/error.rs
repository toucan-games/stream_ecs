use core::fmt::Display;

use derive_more::{Display, From};

use crate::component::bundle::{NotRegisteredError, TryBundleError};

/// The error type which is returned when trying to build a new entity.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy, From)]
#[display(bound = "Err: Display")]
pub enum TryEntityBuildError<Err> {
    /// Component was not registered in the component registry.
    NotRegistered(NotRegisteredError),
    /// Entity registry failed to create new entity.
    #[from(ignore)]
    #[display(fmt = "entity registry failed to create new entity: {_0}")]
    Entities(Err),
}

/// The error type which is returned when trying to build a new entity.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy, From)]
#[display(bound = "EntitiesErr: Display, StorageErr: Display")]
pub enum TryBuildError<EntitiesErr, StorageErr> {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Entity registry failed to create new entity.
    #[from(ignore)]
    #[display(fmt = "entity registry failed to create new entity: {_0}")]
    Entities(EntitiesErr),
    /// Component storage failed to attach a bundle to the entity.
    #[from(ignore)]
    #[display(fmt = "storage failed to attach a component: {_0}")]
    Storage(StorageErr),
}

impl<E, S> From<TryBundleError<S>> for TryBuildError<E, S> {
    fn from(error: TryBundleError<S>) -> Self {
        match error {
            TryBundleError::NotRegistered(error) => Self::NotRegistered(error),
            TryBundleError::Storage(error) => Self::Storage(error),
        }
    }
}
