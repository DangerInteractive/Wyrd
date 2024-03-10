//! code associated with component types and component storage

use error::ComponentWriteError;
use std::fmt::Debug;

pub mod array_component_storage;
pub mod b_tree_map_component_storage;
pub mod error;
pub mod hash_map_component_storage;
pub mod vec_component_storage;

/// a marker type representing a type that can be used as a component
pub trait Component: Sized + Debug {}

/// type capable of storing a set of different components of the same type
/// for different entities
pub trait ComponentStorage<T: Component> {
    /// get a component given the index (entity ID)
    fn get(&self, index: usize) -> Option<&T>;

    /// store a component given the index (entity ID)
    fn insert(&mut self, index: usize, component: T) -> Result<Option<T>, ComponentWriteError>;

    /// delete a component given the index (entity ID)
    fn delete(&mut self, index: usize) -> Result<Option<T>, ComponentWriteError>;
}

/// a fake component type for use in unit tests
#[cfg(test)]
#[derive(Debug, Default)]
pub(crate) struct TestComponent(i32);

#[cfg(test)]
impl Component for TestComponent {}
