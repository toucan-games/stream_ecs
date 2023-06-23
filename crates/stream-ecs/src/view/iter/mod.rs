//! Utilities for iteration over all data of the query.

pub use self::{iter::ViewIter, iter_mut::ViewIterMut, ref_iter::ViewRefIter};

mod iter;
mod iter_mut;
mod ref_iter;
