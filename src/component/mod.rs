//! Provides utilities for components in ECS.

use self::storage::Storage;

pub mod storage;

/// Trait for data that can be attached to the entity.
///
/// This trait must be implemented for the types which implement
/// [`Copy`], [`Send`], [`Sync`] traits and contain no non-static references.
///
/// It implements [`Copy`] trait to ensure that type does not manage some resource
/// because copyable types cannot implement [`Drop`].
pub trait Component: Copy + Send + Sync + 'static {
    /// Type of storage which will be used to store this type of component.
    type Storage: Storage<Item = Self>;
}