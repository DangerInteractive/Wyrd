//! An Entity Component System Library
use std::fmt::Debug;
use std::iter::repeat;

pub trait Component: Sized + Debug {}

pub trait ComponentStorage<T: Component> {
    fn get_component(&self, index: usize) -> Option<&'_ T>;
    fn set_component(&mut self, index: usize, component: T) -> Result<(), ()>;
    fn delete_component(&mut self, index: usize) -> Result<(), ()>;
}

#[derive(Debug, Default)]
pub struct ArrayComponentStorage<T: Component, const SIZE: usize> {
    components: [Option<T>; SIZE],
}

impl<T, const SIZE: usize> ComponentStorage<T> for ArrayComponentStorage<T, SIZE> where T: Component {
    fn get_component(&self, index: usize) -> Option<&'_ T> {
        self.components.get(index)
    }

    fn set_component(&mut self, index: usize, component: T) -> Result<(), ()> {
        match index {
            index if index < SIZE => {
                self.components.get_mut(index) = Some(component);
                Ok(())
            }
            _ => Err(())
        }
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ()> {
        match index {
            index if index < SIZE => {
                self.components.get_mut(index) = None;
                Ok(())
            }
            _ => Err(())
        }
    }
}

#[derive(Debug, Default)]
pub struct VecComponentStorage<T: Component> {
    components: Vec<Option<T>>,
}

impl<T> VecComponentStorage<T> where T: Component {
    pub fn new_with_initial_size(initial_size: usize) -> Self {
        let mut components = vec![];
        components.reserve(initial_size);
        Self {
            components
        }
    }
}

impl<T> ComponentStorage<T> for VecComponentStorage<T> where T: Component {
    fn get_component(&self, index: usize) -> Option<&'_ T> {
        self.components.get(index)
    }

    fn set_component(&mut self, index: usize, component: T) -> Result<(), ()> {
        match self.components.len() {
            len if index < len => {
                self.components.get_mut(index) = Some(component);
                Ok(())
            }
            len => {
                self.components.reserve(index - len);
                self.components.extend(repeat(None).take(index - len));
                self.set_component(index, component)
            }
        }
    }

    fn delete_component(&mut self, index: usize) -> Result<(), ()> {
        if let Some(component) = self.components.get_mut(index) {
            *component = None;
        }
        Ok(())
    }
}
