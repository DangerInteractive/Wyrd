use crate::data_structure::parking_lot;
use crate::data_structure::parking_lot::error::{DeleteError, PutError};
use crate::data_structure::parking_lot::link::Link;
use crate::data_structure::parking_lot::space::Space;
use crate::data_structure::parking_lot::space::Space::Full;
use crate::data_structure::parking_lot::ParkingLot;

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
