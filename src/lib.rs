//! An Entity Component System Library
use std::array::from_fn;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait Component: Sized + Debug {}

#[derive(Debug)]
pub struct ComponentWriteError {
    component_type: &'static str,
    entity_id: usize,
    detail: Option<&'static str>,
}

impl ComponentWriteError {
    pub fn new<T>(entity_id: usize) -> Self {
        Self {
            component_type: type_name::<T>(),
            entity_id,
            detail: None,
        }
    }

    pub fn new_with_detail<T>(entity_id: usize, detail: &'static str) -> Self {
        Self {
            component_type: type_name::<T>(),
            entity_id,
            detail: Some(detail),
        }
    }
}

impl Display for ComponentWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.detail {
            Some(detail) => write!(
                f,
                "Failed to write `{}` component for entity {}: {}",
                self.component_type, self.entity_id, detail
            ),
            None => write!(
                f,
                "Failed to write `{}` component for entity {}.",
                self.component_type, self.entity_id
            ),
        }
    }
}

impl Error for ComponentWriteError {}

pub trait ComponentStorage<T: Component> {
    fn get_component(&self, index: usize) -> Option<&'_ T>;
    fn set_component(&mut self, index: usize, component: T) -> Result<(), ComponentWriteError>;
    fn delete_component(&mut self, index: usize) -> Result<(), ComponentWriteError>;
}

#[derive(Debug)]
pub struct ArrayComponentStorage<T: Component, const SIZE: usize> {
    components: [Option<T>; SIZE],
}

impl<T, const SIZE: usize> ComponentStorage<T> for ArrayComponentStorage<T, SIZE>
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
        if let Some(stored) = self.components.get_mut(index) {
            *stored = Some(component);
            return Ok(());
        }
        Err(ComponentWriteError::new::<T>(index))
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ComponentWriteError> {
        if let Some(stored) = self.components.get_mut(index) {
            *stored = None;
            return Ok(());
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

#[derive(Debug, Default)]
pub struct VecComponentStorage<T: Component> {
    components: Vec<Option<T>>,
}

impl<T> VecComponentStorage<T>
where
    T: Component,
{
    pub fn new_with_initial_size(initial_size: usize) -> Self {
        let mut components = vec![];
        components.reserve(initial_size);
        Self { components }
    }
}

impl<T> VecComponentStorage<T>
where
    T: Component,
{
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
