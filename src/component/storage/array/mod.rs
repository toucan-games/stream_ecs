//! Component storage implementations backed by an array.
//!
//! Such implementations do not use heap allocation at all, so they could be used in `no_std` environment.

pub use self::basic::ArrayStorage;

pub mod basic;

/// The result type which is returned when array registry capacity was exceeded.
pub type ArrayRegistryResult<T> = Result<T, ArrayRegistryError>;

/// The error type which is returned when array registry capacity was exceeded.
#[derive(Debug, Clone, Copy)]
pub struct ArrayRegistryError;

impl core::fmt::Display for ArrayRegistryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "array registry capacity exceeded")
    }
}
