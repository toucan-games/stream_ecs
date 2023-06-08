//! Entity registry implementations backed by an array.
//!
//! Such implementations do not use heap allocation at all, so they could be used in `no_std` environment.

use derive_more::Display;

pub use self::basic::ArrayRegistry;
pub use self::dense::DenseArrayRegistry;

pub mod basic;
pub mod dense;

/// The error type which is returned when array registry capacity was exceeded.
#[derive(Debug, Display, Clone, Copy)]
#[display(fmt = "array registry capacity exceeded")]
pub struct ArrayRegistryError;
