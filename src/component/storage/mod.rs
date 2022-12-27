//! Utilities for [storages](Storage) of components in ECS.

use super::Component;

/// Trait for storages of components in ECS.
pub trait Storage: Send + Sync + 'static {
    /// Type of component which is stored in this storage.
    type Item: Component;
}
