use core::fmt::Display;

use derive_more::{Display, From};

use crate::{
    component::bundle::{NotRegisteredError, TryBundleError},
    entity::registry::NotPresentError,
};

/// The error type which is returned when operating with entities in the ECS world.
#[derive(Debug, Display, Clone, Copy, From)]
pub enum EntityError {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Entity was not present in the world.
    NotPresent(NotPresentError),
}

/// The error type which is returned when trying to attach a bundle to the entity in the world.
#[derive(Debug, Display, Clone, Copy, From)]
#[display(bound = "Err: Display")]
pub enum TryAttachError<Err> {
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Entity was not present in the world.
    NotPresent(NotPresentError),
    /// Component storage failed to attach a bundle to the entity.
    #[from(ignore)]
    #[display(fmt = "storage failed to attach a component: {_0}")]
    Storage(Err),
}

impl<Err> From<TryBundleError<Err>> for TryAttachError<Err> {
    fn from(error: TryBundleError<Err>) -> Self {
        match error {
            TryBundleError::NotRegistered(error) => Self::NotRegistered(error),
            TryBundleError::Storage(error) => Self::Storage(error),
        }
    }
}
