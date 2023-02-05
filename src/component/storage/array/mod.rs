//! Component storage implementations backed by an array.
//!
//! Such implementations do not use heap allocation at all, so they could be used in `no_std` environment.

use core::fmt::Display;

pub use self::basic::ArrayStorage;

pub mod basic;

/// The error type which is returned when array registry capacity was exceeded.
#[derive(Debug, Clone, Copy)]
pub struct ArrayStorageError;

impl Display for ArrayStorageError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "array storage capacity exceeded")
    }
}
