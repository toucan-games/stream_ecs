//! Provides utilities for components in ECS.

use as_any::AsAny;

/// Derive macro for [`Component`] trait.
#[cfg(feature = "derive")]
pub use stream_ecs_macros::Component;

use self::storage::Storage;

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
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait Component: Copy + 'static {
    /// Type of storage which will be used to store this type of component.
    type Storage: Storage<Item = Self>;
}

/// Erased variant of component of some component type in ECS.
///
/// Compared to [`Component`] trait, this trait is guaranteed to be object safe, so it can be used as trait object.
/// This trait is implemented for all the components, so it can be used as trait object for any type of component.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait ErasedComponent: AsAny {}

impl<T> ErasedComponent for T where T: Component {}
