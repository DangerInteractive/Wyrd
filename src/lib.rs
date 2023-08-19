//! An Entity Component System Library
use std::any::{type_name, Any, TypeId};
use std::array::from_fn;
use std::collections::HashMap;
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

#[derive(Debug, Default)]
pub struct EntityStorage {
    component_storages: HashMap<TypeId, Box<dyn Any>>,
}

impl EntityStorage {
    pub fn insert_component_storage<T: 'static + Component, S: 'static + ComponentStorage<T>>(
        &mut self,
        storage: S,
    ) -> Option<Box<dyn ComponentStorage<T>>> {
        let boxed_trait: Box<dyn ComponentStorage<T>> = Box::new(storage);
        let boxed_any: Box<dyn Any> = Box::new(boxed_trait);

        let prev = self.component_storages.insert(TypeId::of::<T>(), boxed_any);
        match prev {
            Some(prev_storage) => {
                if let Ok(s) = prev_storage.downcast::<Box<dyn ComponentStorage<T>>>() {
                    return Some(*s);
                }
                None
            }
            None => None,
        }
    }

    pub fn get_component_storage_ref<T: 'static + Component>(
        &self,
    ) -> Option<&dyn ComponentStorage<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.component_storages.get(&type_id) {
            if let Some(s) = storage.downcast_ref::<Box<dyn ComponentStorage<T>>>() {
                return Some(s.as_ref());
            }
        }
        None
    }
}

#[cfg(test)]
#[derive(Debug, Default)]
struct TestComponent(i32);

#[cfg(test)]
impl Component for TestComponent {}

#[test]
fn entity_storage_can_store_and_retrieve_component_storages() {
    let mut es: EntityStorage = Default::default();
    let cs: VecComponentStorage<TestComponent> = Default::default();
    assert!(
        es.insert_component_storage(cs).is_none(),
        "an existing component storage was returned when one shouldn't have existed yet."
    );
    assert!(
        es.get_component_storage_ref::<TestComponent>().is_some(),
        "entity storage returned None when a storage known to exist was requested."
    );
}

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
