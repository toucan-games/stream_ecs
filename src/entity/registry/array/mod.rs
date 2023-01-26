//! Entity registry implementations backed by an array.
//!
//! Such implementations do not use heap allocation at all, so they could be used in `no_std` environment.

pub use self::basic::ArrayRegistry;

pub mod basic;
