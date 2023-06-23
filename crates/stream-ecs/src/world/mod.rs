//! Provides utilities for ECS worlds.

pub use self::{
    builder::EntityBuilder,
    error::{EntityError, TryAttachError},
    view::{View, ViewRef},
    world::World,
};

mod builder;
mod error;
mod view;
mod world;
