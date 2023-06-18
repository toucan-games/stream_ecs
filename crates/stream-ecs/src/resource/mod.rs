//! Provides utilities for resources in ECS.

use as_any::AsAny;

#[cfg(feature = "derive")]
pub use stream_ecs_macros::Resource;

pub mod bundle;
pub mod registry;

/// Trait for data that can be stored as singleton in ECS
/// which does not belong to any specific entity.
///
/// This trait can be implemented for types which
/// does not contain any non-static references.
///
/// Unlike [components][component], resources does not need to be [copyable][`Copy`]
/// because they are used to share some state across entities and manage some resources.
///
/// Storing and accessing resources can be useful to access unique data in systems.
///
/// [component]: crate::component::Component
pub trait Resource: 'static {}

/// Erased variant of resource of some resource type in ECS.
///
/// Compared to [`Resource`] trait, this trait is guaranteed to be object safe, so it can be used as trait object.
/// This trait is implemented for all the resources, so it can be used as trait object for any type of resource.
pub trait ErasedResource: AsAny {}

impl<T> ErasedResource for T where T: Resource {}
