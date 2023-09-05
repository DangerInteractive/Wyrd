//! an implementation of `ParkingLot` that uses a resizable vector as backing memory

use crate::memory::depot;
use crate::memory::depot::error::{DeleteError, PutError};
use crate::memory::depot::link::Link;
use crate::memory::depot::space::Space;
use crate::memory::depot::space::Space::Full;
use crate::memory::depot::Depot;

/// an implementation of `ParkingLot` that uses a resizable vector as backing memory
pub struct VecDepot<T> {
    next_empty: Link,
    vec: Vec<Space<T>>,
}

impl<T> Depot<T> for VecDepot<T> {
    fn get(&self, index: usize) -> Option<&T> {
        if let Some(Full(value)) = self.vec.get(index) {
            return Some(value);
        }
        None
    }

    fn put(&mut self, value: T) -> Result<usize, PutError> {
        match self.next_empty.0 {
            Some(index) => {
                depot::put_space(&mut self.vec[index], &mut self.next_empty, value);
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
                depot::delete_space(space, &mut self.next_empty, index);
                Ok(())
            }
            None => Err(DeleteError::new_with_detail(index, "not found")),
        }
    }
}

impl<T> Default for VecDepot<T> {
    fn default() -> Self {
        Self {
            next_empty: Link::new_end(),
            vec: vec![],
        }
    }
}

#[test]
fn can_fit_lots_of_elements() {
    let mut depot: VecDepot<usize> = Default::default();
    for i in 0..1000000 {
        assert!(depot.put(i).is_ok(), "failed to put a value");
    }
}

#[test]
fn can_delete_elements() {
    let mut depot: VecDepot<usize> = Default::default();
    let index = depot.put(0);
    match index {
        Ok(index) => {
            assert!(depot.delete(index).is_ok(), "failed to delete a value");
            assert!(depot.get(index).is_none(), "value persisted after deleting");
        }
        Err(_) => panic!("cannot test deletion because insertion failed"),
    }
}
