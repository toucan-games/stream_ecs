//! Entity registry implementations backed by an array.
//!
//! Such implementations do not use heap allocation at all, so they could be used in `no_std` environment.

use core::fmt::Display;

pub use self::basic::ArrayRegistry;
pub use self::dense::DenseArrayRegistry;

pub mod basic;
pub mod dense;

/// The error type which is returned when array registry capacity was exceeded.
#[derive(Debug, Clone, Copy)]
pub struct ArrayRegistryError;

impl Display for ArrayRegistryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "array registry capacity exceeded")
    }
}
