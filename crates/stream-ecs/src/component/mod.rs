//! Provides utilities for components in ECS.

use self::storage::Storage;

#[cfg(feature = "derive")]
pub use stream_ecs_macros::Component;

pub mod bundle;
pub mod registry;
pub mod storage;

/// Trait for data that can be attached to an entity.
///
/// This trait can be implemented for types which implement
/// [`Copy`] trait and contain no non-static references.
///
/// It implements [`Copy`] trait to ensure that type does not manage some resource
/// because copyable types cannot implement [`Drop`].
pub trait Component: Copy + 'static {
    /// Type of storage which will be used to store this type of component.
    type Storage: Storage<Item = Self>;
}
