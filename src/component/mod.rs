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

#[cfg(test)]
pub mod test {
    use crate::component::{Component, ComponentStorage};

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
    /// a fake component type for use in unit tests
    pub struct TestComponent(pub i32);

    impl Component for TestComponent {}

    /// test that a `ComponentStorage` satisfies the expected core initialization behavior
    pub fn test_init_behavior<S: ComponentStorage<TestComponent>, I: Iterator<Item = usize>>(
        storage: &S,
        index_iterator: I,
    ) {
        for i in index_iterator {
            assert!(
                storage.get(i).is_none(),
                "element {} in newly initialized ArrayComponentStorage was not None",
                i
            )
        }
    }

    /// test that a `ComponentStorage` satisfies the expected core insertion and updating behavior
    pub fn test_insert_and_update_behavior<
        S: ComponentStorage<TestComponent>,
        I: Iterator<Item = usize>,
    >(
        storage: &mut S,
        index_iterator: I,
    ) {
        for i in index_iterator {
            let value = i as i32;
            match storage.insert(i, TestComponent(value)) {
                Ok(x) => {
                    if let Some(x) = x {
                        panic!("inserting into empty slot (index: {}) returned Some ({:?}) as the slot's previous value", i, x)
                    }
                }
                Err(err) => panic!(
                    "inserting into empty slot (index: {}) returned an error: {:?}",
                    i, err
                ),
            }
            match storage.get(i) {
                Some(x) => assert_eq!(
                    x.0,
                    value,
                    "value retrieved from index {}: {:?} did not match what was inserted there: {:?}",
                    i,
                    x,
                    TestComponent(value)
                ),
                None => panic!(
                    "attempt to retrieve the value that was inserted at index {} returned None",
                    i
                ),
            }
            let updated_value = ((i + 1) * 64) as i32;
            match storage.insert(i, TestComponent(updated_value)) {
                Ok(x) => match x {
                    Some(x) => assert_eq!(x.0, value, "updating value at index {} returned {:?} as the slot's previous value, expected {:?}", i, x, TestComponent(value)),
                    None => panic!("updating into slot (index: {}) returned None as the slot's previous value, expected {:?}", i, TestComponent(updated_value))
                },
                Err(err) => panic!("updating slot (index: {}) returned an error: {:?}", i, err)
            }
            match storage.get(i) {
                Some(x) => assert_eq!(
                    x.0,
                    updated_value,
                    "value retrieved from index {}: {:?} did not match what was updated there: {:?}",
                    i,
                    x,
                    TestComponent(updated_value)
                ),
                None => panic!(
                    "attempt to retrieve the value that was updated at index {} returned None",
                    i
                ),
            }
        }
    }
}
