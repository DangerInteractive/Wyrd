use std::mem;
use crate::component::error::ComponentWriteError;
use crate::component::{Component, ComponentStorage};

/// an implementation of `ComponentStorage` the uses resizable vectors as backing memory
#[derive(Debug, Default)]
pub struct VecComponentStorage<T: Component> {
    components: Vec<Option<T>>,
}

impl<T> VecComponentStorage<T>
where
    T: Component,
{
    /// create a new `VecComponentStorage` with a certain size reserved
    /// in advance (it can still resize beyond this initial size)
    pub fn new_with_initial_size(initial_size: usize) -> Self {
        Self {
            components: Vec::with_capacity(initial_size),
        }
    }

    /// resize the backing memory, filling new cells with `None`
    fn resize(&mut self, min_size: usize) {
        let current_length = self.components.len();
        if current_length < min_size {
            self.components.resize_with(min_size, || None);
        }
    }
}

impl<T> ComponentStorage<T> for VecComponentStorage<T>
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
        if index >= self.components.len() {
            self.resize(index + 1)
        }
        if let Some(stored) = self.components.get_mut(index) {
            let mut tmp = Some(component);
            mem::swap(stored, &mut tmp);
            return Ok(tmp);
        }
        Err(ComponentWriteError::new_with_detail::<T>(index, "index out of bounds"))
    }

    fn delete(&mut self, index: usize) -> Result<Option<T>, ComponentWriteError> {
        if let Some(component) = self.components.get_mut(index) {
            let mut tmp = None;
            mem::swap(component, &mut tmp);
            return Ok(tmp);
        }
        Ok(None) // index out of bounds, but that's okay because we're "deleting" it
    }
}

#[cfg(test)]
mod test {
    use crate::component::test::{
        test_init_behavior, test_insert_and_update_behavior, TestComponent,
    };
    use crate::component::vec_component_storage::VecComponentStorage;

    #[test]
    fn test_init() {
        let storage: VecComponentStorage<TestComponent> = VecComponentStorage::default();
        test_init_behavior(&storage, 0..64);
    }

    #[test]
    fn test_insert_update() {
        let mut storage: VecComponentStorage<TestComponent> =
            VecComponentStorage::default();
        test_insert_and_update_behavior(&mut storage, 0..64);
    }
}
