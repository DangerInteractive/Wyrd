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

#[cfg(test)]
mod test {
    use crate::component::b_tree_map_component_storage::BTreeMapComponentStorage;
    use crate::component::test::{
        test_init_behavior, test_insert_and_update_behavior, TestComponent,
    };

    #[test]
    fn test_init() {
        let storage: BTreeMapComponentStorage<TestComponent> = BTreeMapComponentStorage::default();
        test_init_behavior(&storage, 0..64);
    }

    #[test]
    fn test_insert_update() {
        let mut storage: BTreeMapComponentStorage<TestComponent> =
            BTreeMapComponentStorage::default();
        test_insert_and_update_behavior(&mut storage, 0..64);
    }
}
