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
        let mut components = vec![];
        components.reserve(initial_size);
        Self { components }
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
    fn get_component(&self, index: usize) -> Option<&'_ T> {
        if let Some(Some(component)) = self.components.get(index) {
            return Some(component);
        }
        None
    }

    fn set_component(&mut self, index: usize, component: T) -> Result<(), ComponentWriteError> {
        if index >= self.components.len() {
            self.resize(index + 1)
        }
        if let Some(stored) = self.components.get_mut(index) {
            *stored = Some(component);
        }
        Ok(())
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ComponentWriteError> {
        if let Some(component) = self.components.get_mut(index) {
            *component = None;
        }
        Ok(())
    }
}
