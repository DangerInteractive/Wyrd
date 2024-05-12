//! code associated with managing and composing entities
use crate::component::error::ComponentWriteError;
use crate::component::{Component, ComponentStorage};
use crate::entity::component_storage_set::ComponentStorageSet;
use armory::Depot;
use std::collections::HashMap;
use std::hash::Hash;

pub mod component_storage_set;

pub struct EntityProvision<ID: Clone + Eq + Hash> {
    pub index: usize,
    pub id: ID,
}

pub struct Entity<'a, ID: Clone + Eq + Hash, EntDepot: Depot<ID>, IDGen: Fn() -> ID> {
    pub index: usize,
    world: &'a World<ID, EntDepot, IDGen>,
}

impl<'a, ID, EntDepot, IDGen> Entity<'a, ID, EntDepot, IDGen>
where
    ID: Clone + Eq + Hash,
    EntDepot: Depot<ID>,
    IDGen: Fn() -> ID,
{
    pub fn get_component<T: 'static + Component>(&self) -> Option<&T> {
        if let Some(storage) = self
            .world
            .component_storage_set
            .get_component_storage_ref::<T>()
        {
            return storage.get(self.index);
        }

        None
    }
}

pub struct World<ID, EntDepot, IDGen>
where
    ID: Clone + Eq + Hash,
    EntDepot: Depot<ID>,
    IDGen: Fn() -> ID,
{
    ids: EntDepot,
    id_to_index: HashMap<ID, usize>,
    component_storage_set: ComponentStorageSet,
    entity_id_generator: IDGen,
}

impl<ID, EntDepot, IDGen> World<ID, EntDepot, IDGen>
where
    ID: Clone + Eq + Hash,
    EntDepot: Depot<ID>,
    IDGen: Fn() -> ID,
{
    pub fn register_component<T: 'static + Component, Storage: 'static + ComponentStorage<T>>(
        &mut self,
        component_storage: Storage,
    ) {
        self.component_storage_set
            .insert_component_storage(component_storage);
    }

    pub fn insert_component<T: 'static + Component>(
        &mut self,
        index: usize,
        component: T,
    ) -> Result<Option<T>, ComponentWriteError> {
        if let Some(storage) = self.component_storage_set.get_component_storage_mut::<T>() {
            return storage.insert(index, component);
        }

        Err(ComponentWriteError::new_with_detail::<T>(
            index,
            "component storage not found",
        ))
    }

    pub fn provision_entity(&mut self) -> Result<EntityProvision<ID>, String> {
        let id = (self.entity_id_generator)();
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
