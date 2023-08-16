//! Provides a builder pattern implementation for entities.

pub use self::{
    builder::EntityBuilder,
    error::{TryBuildError, TryEntityBuildError},
    with::With,
};

mod builder;
mod error;
mod with;
