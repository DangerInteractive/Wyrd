use crate::component::{Component, ComponentStorage};
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[cfg(test)]
use crate::component::test::TestComponent;
#[cfg(test)]
use crate::component::vec_component_storage::VecComponentStorage;

/// a collection to store the different `ComponentStorage`s for different component types
#[derive(Debug, Default)]
pub struct ComponentStorageSet {
    component_storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStorageSet {
    /// store a new `ComponentStorage` for a new component type
    /// (storing a `ComponentStorage` for a component already stored will
    /// overwrite the previous one)
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

    /// get a reference to the `ComponentStorage` for a component type (as a `ComponentStorage` trait object only)
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

    /// get a mutable reference to the `ComponentStorage` for a component type (as a `ComponentStorage` trait object only)
    pub fn get_component_storage_mut<T: 'static + Component>(
        &mut self,
    ) -> Option<&mut dyn ComponentStorage<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.component_storages.get_mut(&type_id) {
            if let Some(s) = storage.downcast_mut::<Box<dyn ComponentStorage<T>>>() {
                return Some(s.as_mut());
            }
        }
        None
    }
}

#[test]
fn can_store_and_retrieve_component_storages() {
    let mut component_storage_set: ComponentStorageSet = Default::default();
    let component_storage: VecComponentStorage<TestComponent> = Default::default();
    assert!(
        component_storage_set
            .insert_component_storage(component_storage)
            .is_none(),
        "an existing component storage was returned when one shouldn't have existed yet."
    );
    assert!(
        component_storage_set
            .get_component_storage_ref::<TestComponent>()
            .is_some(),
        "entity storage returned None when a storage known to exist was requested."
    );
}
