//! an implementation of `ParkingLot` that uses a resizable vector as backing memory

use crate::memory::parking_lot;
use crate::memory::parking_lot::error::{DeleteError, PutError};
use crate::memory::parking_lot::link::Link;
use crate::memory::parking_lot::space::Space;
use crate::memory::parking_lot::space::Space::Full;
use crate::memory::parking_lot::ParkingLot;

/// an implementation of `ParkingLot` that uses a resizable vector as backing memory
pub struct VecParkingLot<T> {
    next_empty: Link,
    vec: Vec<Space<T>>,
}

impl<T> ParkingLot<T> for VecParkingLot<T> {
    fn get(&self, index: usize) -> Option<&T> {
        if let Some(Full(value)) = self.vec.get(index) {
            return Some(value);
        }
        None
    }

    fn put(&mut self, value: T) -> Result<usize, PutError> {
        match self.next_empty.0 {
            Some(index) => {
                parking_lot::put_space(&mut self.vec[index], &mut self.next_empty, value);
                Ok(index)
            }
            None => {
                self.vec.push(Full(value));
                Ok(self.vec.len() - 1)
            }
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), DeleteError> {
        match self.vec.get_mut(index) {
            Some(space) => {
                parking_lot::delete_space(space, &mut self.next_empty, index);
                Ok(())
            }
            None => Err(DeleteError::new_with_detail(index, "not found")),
        }
    }
}

impl<T> Default for VecParkingLot<T> {
    fn default() -> Self {
        Self {
            next_empty: Link::new_end(),
            vec: vec![],
        }
    }
}

#[test]
fn vec_parking_lot_can_fit_lots_of_elements() {
    let mut parking_lot: VecParkingLot<usize> = Default::default();
    for i in 0..1000000 {
        assert!(parking_lot.put(i).is_ok(), "failed to put a value");
    }
}

#[test]
fn vec_parking_lot_can_delete_elements() {
    let mut parking_lot: VecParkingLot<usize> = Default::default();
    let index = parking_lot.put(0);
    match index {
        Ok(index) => {
            assert!(
                parking_lot.delete(index).is_ok(),
                "failed to delete a value"
            );
            assert!(
                parking_lot.get(index).is_none(),
                "value persisted after deleting"
            );
        }
        Err(_) => panic!("cannot test deletion because insertion failed"),
    }
}
