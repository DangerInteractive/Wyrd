use crate::data_structure::parking_lot;
use crate::data_structure::parking_lot::error::{DeleteError, PutError};
use crate::data_structure::parking_lot::link::Link;
use crate::data_structure::parking_lot::space::Space;
use crate::data_structure::parking_lot::space::Space::{Empty, Full};
use crate::data_structure::parking_lot::ParkingLot;
use std::array::from_fn;

pub struct ArrayParkingLot<T, const SIZE: usize> {
    next_empty: Link,
    array: [Space<T>; SIZE],
}

impl<T, const SIZE: usize> Default for ArrayParkingLot<T, SIZE> {
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

impl<T, const SIZE: usize> ParkingLot<T> for ArrayParkingLot<T, SIZE> {
    fn get(&self, index: usize) -> Option<&T> {
        if let Some(Full(value)) = self.array.get(index) {
            return Some(value);
        }
        None
    }

    fn put(&mut self, value: T) -> Result<usize, PutError> {
        match self.next_empty.0 {
            Some(index) => {
                parking_lot::put_space(&mut self.array[index], &mut self.next_empty, value);
                Ok(index)
            }
            None => Err(PutError::new_with_detail("out of space")),
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), DeleteError> {
        match self.array.get_mut(index) {
            Some(space) => {
                parking_lot::delete_space(space, &mut self.next_empty, index);
                Ok(())
            }
            None => Err(DeleteError::new_with_detail(index, "not found")),
        }
    }
}

#[test]
fn array_parking_lot_can_fit_exactly_n_elements() {
    let mut parking_lot: ArrayParkingLot<usize, 20> = Default::default();
    for i in 0..20 {
        assert!(matches!(parking_lot.put(i), Ok(_)));
    }
    assert!(matches!(parking_lot.put(21), Err(_)));
}

#[test]
fn array_parking_lot_can_delete_elements() {
    let mut parking_lot: ArrayParkingLot<usize, 1> = Default::default();
    let index = parking_lot.put(0);
    match index {
        Ok(index) => {
            assert!(matches!(parking_lot.delete(index), Ok(_)));
            assert!(parking_lot.get(index).is_none());
        }
        Err(_) => panic!("cannot test deletion because insertion failed"),
    }
}
