use crate::component::Component;
use std::any::TypeId;
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

#[derive(Clone, Default, Eq, Hash, PartialEq, Debug)]
pub struct ArchetypeType(BTreeSet<TypeId>);

impl ArchetypeType {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with<T: Component + 'static>(mut self) -> Self {
        self.0.insert(TypeId::of::<T>());
        self
    }

    pub fn has<T: Component + 'static>(&self) -> bool {
        self.0.contains(&TypeId::of::<T>())
    }
}

pub struct Archetype {
    archetype_type: ArchetypeType,
    component_storages: HashMap<TypeId, *mut u8>,
}
