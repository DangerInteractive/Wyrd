use crate::component::error::ComponentWriteError;
use crate::component::{Component, ComponentStorage};
use std::collections::HashMap;

pub struct HashMapComponentStorage<T: Component> {
    components: HashMap<usize, T>,
}

impl<T> HashMapComponentStorage<T>
where
    T: Component,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> ComponentStorage<T> for HashMapComponentStorage<T>
where
    T: Component,
{
    fn get_component(&self, index: usize) -> Option<&T> {
        self.components.get(&index)
    }

    fn set_component(&mut self, index: usize, component: T) -> Result<(), ComponentWriteError> {
        if self.components.insert(index, component).is_some() {
            return Ok(());
        }
        Err(ComponentWriteError::new_with_detail::<T>(
            index,
            "failed to insert component into HashMap",
        ))
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ComponentWriteError> {
        if self.components.remove(&index).is_some() {
            return Ok(());
        }
        Err(ComponentWriteError::new_with_detail::<T>(
            index,
            "failed to remove component from HashMap",
        ))
    }
}

impl<T> Default for HashMapComponentStorage<T>
where
    T: Component,
{
    fn default() -> Self {
        Self {
            components: Default::default(),
        }
    }
}
