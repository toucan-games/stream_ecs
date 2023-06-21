//! Provides utilities for views of entities and their components in ECS.

// TODO view API

pub use self::view::View;

pub mod iter;
pub mod query;

mod view;
