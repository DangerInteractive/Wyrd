//! a data structure that enables quick random access/insertion/removal of
//! unordered items without reallocating or shifting elements

use error::{DeleteError, PutError};
use link::Link;
use space::Space;
use space::Space::{Empty, Full};
use std::mem::replace;

pub mod array_depot;
pub mod error;
pub mod link;
pub mod space;
pub mod vec_depot;

/// depot internals: ut a value into a space and update the next empty link
pub fn put_space<T>(space: &mut Space<T>, next_empty: &mut Link, value: T) {
    *next_empty =
        replace(space, Full(value)).expect_empty("next_empty pointed to a non-empty space");
}

/// depot internals: delete a value from a space and update the next empty link
pub fn delete_space<T>(space: &mut Space<T>, next_empty: &mut Link, index: usize) {
    if space.is_full() {
        let link = Link::push_new(index, next_empty);
        *space = Empty(link);
    }
}

/// capability to store values in a depot data structure implementation
pub trait Depot<T> {
    /// get a value from the depot given its index
    fn get(&self, index: usize) -> Option<&T>;

    /// store a value in the depot, returning its assigned index
    fn put(&mut self, value: T) -> Result<usize, PutError>;

    /// delete a value from the depot given its index
    fn delete(&mut self, index: usize) -> Result<(), DeleteError>;
}
