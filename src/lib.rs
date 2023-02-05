//! Yet another ECS implementation.

// TODO proper crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

pub mod component;
pub mod entity;
pub mod resource;
pub mod world;
