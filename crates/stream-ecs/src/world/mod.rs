//! Provides utilities for ECS worlds.

pub use self::{
    error::{EntityError, TryAttachError},
    view::{View, ViewRef},
    world::World,
};

mod error;
mod view;
mod world;
