//! code associated with managing and composing entities
use crate::component::error::ComponentWriteError;
use crate::component::{Component, ComponentStorage};
use crate::entity::component_storage_set::ComponentStorageSet;
use armory::Depot;
use std::collections::HashMap;
use std::hash::Hash;

pub mod component_storage_set;

pub trait EntityIdGenerator<T: Clone + Eq + Hash> {
    fn generate_entity_id(&self) -> T;
}

pub struct EntityProvision<T: Clone + Eq + Hash> {
    pub index: usize,
    pub id: T,
}

pub struct World<I, D, G>
where
    I: Clone + Eq + Hash,
    D: Depot<I>,
    G: EntityIdGenerator<I>,
{
    ids: D,
    id_to_index: HashMap<I, usize>,
    component_storage_set: ComponentStorageSet,
    entity_id_generator: G,
}

impl<I, D, G> World<I, D, G>
where
    I: Clone + Eq + Hash,
    D: Depot<I>,
    G: EntityIdGenerator<I>,
{
    pub fn register_component<T: 'static + Component, S: 'static + ComponentStorage<T>>(
        &mut self,
        component_storage: S,
    ) {
        self.component_storage_set
            .insert_component_storage(component_storage);
    }

    pub fn insert_component<T: 'static + Component>(
        &mut self,
        index: usize,
        component: T,
    ) -> Result<(), ComponentWriteError> {
        if let Some(storage) = self.component_storage_set.get_component_storage_mut::<T>() {
            return storage.set_component(index, component);
        }

        Err(ComponentWriteError::new_with_detail::<T>(
            index,
            "component storage not found",
        ))
    }

    pub fn provision_entity(&mut self) -> Result<EntityProvision<I>, String> {
        let id = self.entity_id_generator.generate_entity_id();
        match self.ids.put(id.clone()) {
            Ok(index) => {
                if self.id_to_index.insert(id.clone(), index).is_none() {
                    return Err(String::from(
                        "failed to insert entry to map entity ID to entity index",
                    ));
                }
                Ok(EntityProvision { index, id })
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
