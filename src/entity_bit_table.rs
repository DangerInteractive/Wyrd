pub trait EntityBitTable {
    fn get_entity_flags(&self, entity_id: usize) -> Option<&u64>;
    fn entity_has_component(&self, entity_id: usize, component_id: usize) -> bool {
        if let Some(entity_flags) = self.get_entity_flags(entity_id) {
            return (entity_flags & (1 << component_id)) != 0;
        }
        false
    }
    fn entity_has_components(&self, entity_id: usize, component_ids: &[usize]) -> bool {
        component_ids
            .iter()
            .all(|component_id| self.entity_has_component(entity_id, *component_id))
    }

    fn entity_lacks_components(&self, entity_id: usize, component_ids: &[usize]) -> bool {
        component_ids
            .iter()
            .all(|component_id| !self.entity_has_component(entity_id, *component_id))
    }
}

pub struct VecEntityBitTable {
    entity_table: Vec<u64>,
}

impl EntityBitTable for VecEntityBitTable {
    fn get_entity_flags(&self, entity_id: usize) -> Option<&u64> {
        self.entity_table.get(entity_id)
    }
}

pub struct ArrayEntityBitTable<const SIZE: usize> {
    entity_table: [u64; SIZE],
}

impl<const SIZE: usize> EntityBitTable for ArrayEntityBitTable<SIZE> {
    fn get_entity_flags(&self, entity_id: usize) -> Option<&u64> {
        self.entity_table.get(entity_id)
    }
}
