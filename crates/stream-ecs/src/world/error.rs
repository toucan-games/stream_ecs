use derive_more::{Display, From};

use crate::{
    component::bundle::{NotRegisteredError, TryBundleError},
    entity::{DefaultEntity, Entity, registry::NotPresentError},
};

/// The error type which is returned when operating with entities in the ECS world.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy, From)]
#[display(bound(E: Display))]
pub enum EntityError<E = DefaultEntity>
where
    E: Entity,
{
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Entity was not present in the world.
    NotPresent(NotPresentError<E>),
}

/// The error type which is returned when trying to attach a bundle to the entity in the world.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy, From)]
#[display(bound(Err: Display, E: Display))]
pub enum TryAttachError<Err, E = DefaultEntity>
where
    E: Entity,
{
    /// Component was not registered in the world.
    NotRegistered(NotRegisteredError),
    /// Entity was not present in the world.
    NotPresent(NotPresentError<E>),
    /// Component storage failed to attach a bundle to the entity.
    #[from(ignore)]
    #[display("storage failed to attach a component: {_0}")]
    Storage(Err),
}

impl<Err, E> From<TryBundleError<Err>> for TryAttachError<Err, E>
where
    E: Entity,
{
    fn from(error: TryBundleError<Err>) -> Self {
        match error {
            TryBundleError::NotRegistered(error) => Self::NotRegistered(error),
            TryBundleError::Storage(error) => Self::Storage(error),
        }
    }
}
