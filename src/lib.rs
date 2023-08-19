//! An Entity Component System Library
use std::array::from_fn;
use std::cmp::max;
use std::fmt::Debug;
use std::iter::repeat_with;

pub trait Component: Sized + Debug {}

pub enum ComponentStorageError {}

pub trait ComponentStorage<T: Component> {
    fn get_component(&self, index: usize) -> Option<&'_ T>;
    fn set_component(&mut self, index: usize, component: T) -> Result<(), ()>;
    fn delete_component(&mut self, index: usize) -> Result<(), ()>;
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

    fn set_component(&mut self, index: usize, component: T) -> Result<(), ()> {
        if let Some(stored) = self.components.get_mut(index) {
            *stored = Some(component);
            return Ok(());
        }
        Err(())
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ()> {
        if let Some(stored) = self.components.get_mut(index) {
            *stored = None;
            return Ok(());
        }
        Err(())
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
    fn resize(&mut self, min_size: usize) -> Result<(), ()> {
        let additional = max(0, min_size - self.components.len());
        if additional > 0 {
            self.components.reserve(additional);
            self.components
                .extend(repeat_with(|| None).take(additional));
        }
        Ok(())
    }
}

impl<T> ComponentStorage<T> for VecComponentStorage<T>
where
    T: Component,
{
    fn get_component(&self, index: usize) -> Option<&'_ T> {
        match self.components.get(index) {
            Some(Some(component)) => Some(component),
            _ => None,
        }
    }

    fn set_component(&mut self, index: usize, component: T) -> Result<(), ()> {
        if index >= self.components.len() {
            self.resize(index + 1)?
        }
        if let Some(stored) = self.components.get_mut(index) {
            *stored = Some(component);
        }
        Ok(())
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ()> {
        if let Some(component) = self.components.get_mut(index) {
            *component = None;
        }
        Ok(())
    }
}
