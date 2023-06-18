//! Yet another ECS implementation.

// TODO proper crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

pub use ::hlist;

pub mod component;
pub mod entity;
pub mod resource;
pub mod view;
pub mod world;

mod ref_mut;
mod registry;
