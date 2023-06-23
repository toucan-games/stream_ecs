//! Provides utilities for views of entities and their components in ECS.

pub use self::{view::View, view_ref::ViewRef};

pub mod iter;
pub mod query;

mod view;
mod view_ref;
