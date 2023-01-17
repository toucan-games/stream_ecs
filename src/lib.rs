//! Yet another ECS implementation.

// TODO proper crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

pub mod component;
pub mod entity;
pub mod resource;
