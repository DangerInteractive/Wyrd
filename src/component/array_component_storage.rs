use crate::component::error::ComponentWriteError;
use crate::component::{Component, ComponentStorage};
use std::array::from_fn;
use std::mem;

/// an implementation of `ComponentStorage` that uses fixed-size arrays as backing memory
#[derive(Debug)]
pub struct ArrayComponentStorage<T: Component, const SIZE: usize> {
    components: [Option<T>; SIZE],
}

impl<T, const SIZE: usize> ComponentStorage<T> for ArrayComponentStorage<T, SIZE>
where
    T: Component,
{
    fn get(&self, index: usize) -> Option<&'_ T> {
        if let Some(Some(component)) = self.components.get(index) {
            return Some(component);
        }
        None
    }

    fn insert(&mut self, index: usize, component: T) -> Result<Option<T>, ComponentWriteError> {
        if let Some(stored) = self.components.get_mut(index) {
            let mut tmp = Some(component);
            mem::swap(stored, &mut tmp);
            return Ok(tmp);
        }
        Err(ComponentWriteError::new::<T>(index))
    }

    fn delete(&mut self, index: usize) -> Result<Option<T>, ComponentWriteError> {
        if let Some(stored) = self.components.get_mut(index) {
            let mut tmp = None;
            mem::swap(stored, &mut tmp);
            return Ok(tmp);
        }
        Err(ComponentWriteError::new::<T>(index))
    }
}

impl<T, const SIZE: usize> Default for ArrayComponentStorage<T, SIZE>
where
    T: Component,
{
    fn default() -> Self {
        Self {
            components: from_fn(|_| None),
        }
    }
}
