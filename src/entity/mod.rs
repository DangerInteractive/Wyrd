use crate::component::archetype::{Archetype, ArchetypeType};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Default)]
pub struct Fabric {
    archetypes: HashMap<ArchetypeType, RwLock<Archetype>>,
}

impl Fabric {
    pub fn new() -> Self {
        Default::default()
    }
}
