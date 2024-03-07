use crate::component::error::ComponentWriteError;
use crate::component::{Component, ComponentStorage};
use std::collections::BTreeMap;

pub struct BTreeMapComponentStorage<T: Component> {
    components: BTreeMap<usize, T>,
}

impl<T> BTreeMapComponentStorage<T>
where
    T: Component,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> ComponentStorage<T> for BTreeMapComponentStorage<T>
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
            "failed to insert component into BTreeMap",
        ))
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ComponentWriteError> {
        if self.components.remove(&index).is_some() {
            return Ok(());
        }
        Err(ComponentWriteError::new_with_detail::<T>(
            index,
            "failed to remove component from BTreeMap",
        ))
    }
}

impl<T> Default for BTreeMapComponentStorage<T>
where
    T: Component,
{
    fn default() -> Self {
        Self {
            components: Default::default(),
        }
    }
}
