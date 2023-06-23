//! Provides a builder pattern implementation for entities.

pub use self::{
    builder::EntityBuilder,
    error::{TryBuildError, TryEntityBuildError},
};

mod builder;
mod error;
