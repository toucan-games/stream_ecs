//! Provides utilities for resources in ECS.

pub mod registry;

/// Trait for data that can be stored as singleton in ECS
/// which does not belong to any specific entity.
///
/// This trait can be implemented for types which implement [`Send`], [`Sync`] traits
/// and doesn't contain any non-static references.
///
/// Unlike [components][component], resources does not need to be [copyable][`Copy`]
/// because they are used to share some state across entities and manage some resources.
///
/// Storing and accessing resources can be useful to access unique data in systems.
///
/// [component]: crate::component::Component
pub trait Resource: Send + Sync + 'static {}
