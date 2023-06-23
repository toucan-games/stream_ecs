//! Provides utilities for ECS worlds.

pub use self::{
    error::{EntityError, TryAttachError},
    world::World,
};

mod error;
mod world;
