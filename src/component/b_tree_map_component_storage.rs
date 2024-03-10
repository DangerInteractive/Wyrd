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
    fn get(&self, index: usize) -> Option<&T> {
        self.components.get(&index)
    }

    fn insert(&mut self, index: usize, component: T) -> Result<Option<T>, ComponentWriteError> {
        Ok(self.components.insert(index, component))
    }

    fn delete(&mut self, index: usize) -> Result<Option<T>, ComponentWriteError> {
        Ok(self.components.remove(&index))
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
