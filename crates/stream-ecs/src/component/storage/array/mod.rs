//! Component storage implementations backed by an array.
//!
//! Such implementations do not use heap allocation at all, so they could be used in `no_std` environment.

use derive_more::Display;

pub use self::basic::ArrayStorage;
pub use self::dense::DenseArrayStorage;
pub use self::hash::HashArrayStorage;

pub mod basic;
pub mod dense;
pub mod hash;

/// The error type which is returned when array storage capacity was exceeded.
#[derive(Debug, Display, Clone, Copy)]
#[display(fmt = "array storage capacity exceeded")]
pub struct ArrayStorageError;
