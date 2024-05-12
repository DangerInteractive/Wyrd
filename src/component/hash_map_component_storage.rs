use crate::component::error::ComponentWriteError;
use crate::component::{Component, ComponentStorage};
use std::collections::HashMap;

pub struct HashMapComponentStorage<T: Component> {
    components: HashMap<usize, T>,
}

impl<T> HashMapComponentStorage<T>
where
    T: Component,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> ComponentStorage<T> for HashMapComponentStorage<T>
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

impl<T> Default for HashMapComponentStorage<T>
where
    T: Component,
{
    fn default() -> Self {
        Self {
            components: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::component::hash_map_component_storage::HashMapComponentStorage;
    use crate::component::test::{
        test_init_behavior, test_insert_and_update_behavior, TestComponent,
    };
    use crate::component::ComponentStorage;

    #[test]
    fn test_init() {
        let storage: HashMapComponentStorage<TestComponent> = HashMapComponentStorage::default();
        test_init_behavior(&storage, 0..64);
    }

    #[test]
    fn test_insert_update() {
        let mut storage: HashMapComponentStorage<TestComponent> =
            HashMapComponentStorage::default();
        test_insert_and_update_behavior(&mut storage, 0..64);
    }

    #[test]
    fn test_get_at_arbitrary_index() {
        let storage: HashMapComponentStorage<TestComponent> = HashMapComponentStorage::default();
        if let Some(x) = storage.get(123456789) {
            panic!("retrieved value was {:?}, expected None", x);
        }
    }

    #[test]
    fn test_insert_at_arbitrary_index() {
        let mut storage: HashMapComponentStorage<TestComponent> =
            HashMapComponentStorage::default();
        let test_component = TestComponent(123);
        match storage.insert(123456789, test_component) {
            Ok(x) => {
                if x.is_some() {
                    panic!("inserting returned {:?}, None expected", x)
                }
            }
            Err(err) => panic!("inserting returned error {:?}", err),
        }
        match storage.get(123456789) {
            Some(x) => assert_eq!(
                x.0, test_component.0,
                "retrieved value previously inserted was {:?}, expected {:?}",
                x, test_component
            ),
            None => panic!(
                "retrieved value previously inserted was None, expected {:?}",
                test_component
            ),
        }
    }
}
