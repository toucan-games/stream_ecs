//! Yet another ECS implementation.

// TODO proper crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

pub use ::{hlist, lending_iterator, ref_kind};

pub mod component;
pub mod dependency;
pub mod entity;
pub mod resource;
pub mod view;
pub mod world;

mod utils;
