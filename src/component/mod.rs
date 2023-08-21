//! code associated with component types and component storage

use error::ComponentWriteError;
use std::fmt::Debug;

pub mod array_component_storage;
pub mod component_storage_set;
pub mod error;
pub mod vec_component_storage;

/// a marker type representing a type that can be used as a component
pub trait Component: Sized + Debug {}

/// type capable of storing a set of different components of the same type
/// for different entities
pub trait ComponentStorage<T: Component> {
    /// get a component given the index (entity ID)
    fn get_component(&self, index: usize) -> Option<&T>;

    /// store a component given the index (entity ID)
    fn set_component(&mut self, index: usize, component: T) -> Result<(), ComponentWriteError>;

    /// delete a component given the index (entity ID)
    fn delete_component(&mut self, index: usize) -> Result<(), ComponentWriteError>;
}

/// a fake component type for use in unit tests
#[cfg(test)]
#[derive(Debug, Default)]
struct TestComponent(i32);

#[cfg(test)]
impl Component for TestComponent {}
