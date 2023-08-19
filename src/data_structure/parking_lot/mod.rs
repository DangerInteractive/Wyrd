use error::{DeleteError, PutError};
use link::Link;
use space::Space;
use space::Space::{Empty, Full};
use std::mem::replace;

pub mod array_parking_lot;
pub mod error;
pub mod link;
pub mod space;
pub mod vec_parking_lot;

pub fn put_space<T>(space: &mut Space<T>, next_empty: &mut Link, value: T) {
    *next_empty =
        replace(space, Full(value)).expect_empty("next_empty pointed to a non-empty space");
}

pub fn delete_space<T>(space: &mut Space<T>, next_empty: &mut Link, index: usize) {
    if space.is_full() {
        let link = Link::push_new(index, next_empty);
        *space = Empty(link);
    }
}

pub trait ParkingLot<T> {
    fn get(&self, index: usize) -> Option<&T>;
    fn put(&mut self, value: T) -> Result<usize, PutError>;
    fn delete(&mut self, index: usize) -> Result<(), DeleteError>;
}
