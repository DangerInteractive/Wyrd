//! an implementation of `ParkingLot` that uses a fixed-size array as backing memory

use crate::memory::depot;
use crate::memory::depot::error::{DeleteError, PutError};
use crate::memory::depot::link::Link;
use crate::memory::depot::space::Space;
use crate::memory::depot::space::Space::{Empty, Full};
use crate::memory::depot::Depot;
use std::array::from_fn;

/// an implementation of `ParkingLot` that uses a fixed-size array as backing memory
pub struct ArrayDepot<T, const SIZE: usize> {
    next_empty: Link,
    array: [Space<T>; SIZE],
}

impl<T, const SIZE: usize> Default for ArrayDepot<T, SIZE> {
    fn default() -> Self {
        Self {
            next_empty: Link::new(0),
            array: from_fn(|index| match index {
                index if index == SIZE - 1 => Empty(Link::new_end()),
                index => Empty(Link::new(index + 1)),
            }),
        }
    }
}

impl<T, const SIZE: usize> Depot<T> for ArrayDepot<T, SIZE> {
    fn get(&self, index: usize) -> Option<&T> {
        if let Some(Full(value)) = self.array.get(index) {
            return Some(value);
        }
        None
    }

    fn put(&mut self, value: T) -> Result<usize, PutError> {
        match self.next_empty.0 {
            Some(index) => {
                depot::put_space(&mut self.array[index], &mut self.next_empty, value);
                Ok(index)
            }
            None => Err(PutError::new_with_detail("out of space")),
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), DeleteError> {
        match self.array.get_mut(index) {
            Some(space) => {
                depot::delete_space(space, &mut self.next_empty, index);
                Ok(())
            }
            None => Err(DeleteError::new_with_detail(index, "not found")),
        }
    }
}

#[test]
fn can_fit_exactly_n_elements() {
    let mut depot: ArrayDepot<usize, 20> = Default::default();
    for i in 0..20 {
        assert!(depot.put(i).is_ok(), "failed to put a value");
    }
    assert!(
        depot.put(21).is_err(),
        "successfully put a value when the array should have run out of memory"
    );
}

#[test]
fn can_delete_elements() {
    let mut depot: ArrayDepot<usize, 1> = Default::default();
    let index = depot.put(0);
    match index {
        Ok(index) => {
            assert!(depot.delete(index).is_ok(), "failed to delete a value");
            assert!(depot.get(index).is_none(), "value persisted after deleting");
        }
        Err(_) => panic!("cannot test deletion because insertion failed"),
    }
}
