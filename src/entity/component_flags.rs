//! code for quickly looking up which entities have the required components to process a system

use std::ops::{BitAnd, Shl};

pub trait ComponentFlags<T>
where
    T: Copy + BitAnd<T> + Shl<usize, Output = T> + From<u8>,
    <T as BitAnd<T>>::Output: PartialEq<T>,
{
    fn get_flags_for_entity(&self, entity_id: usize) -> Option<&T>;
    fn entity_has_component(&self, entity_id: usize, component_id: usize) -> bool {
        if let Some(entity_flags) = self.get_flags_for_entity(entity_id) {
            return (*entity_flags & (T::from(1) << component_id)) != T::from(0);
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

pub struct VecComponentFlags<T>
where
    T: Copy + BitAnd<T> + Shl<usize, Output = T> + From<u8>,
{
    entity_table: Vec<T>,
}

impl<T> ComponentFlags<T> for VecComponentFlags<T>
where
    T: Copy + BitAnd<T> + Shl<usize, Output = T> + From<u8>,
    <T as BitAnd<T>>::Output: PartialEq<T>,
{
    fn get_flags_for_entity(&self, entity_id: usize) -> Option<&T> {
        self.entity_table.get(entity_id)
    }
}

pub struct ArrayComponentFlags<T, const SIZE: usize> {
    entity_table: [T; SIZE],
}

impl<T, const SIZE: usize> ComponentFlags<T> for ArrayComponentFlags<T, SIZE>
where
    T: Copy + BitAnd<T> + Shl<usize, Output = T> + From<u8>,
    <T as BitAnd<T>>::Output: PartialEq<T>,
{
    fn get_flags_for_entity(&self, entity_id: usize) -> Option<&T> {
        self.entity_table.get(entity_id)
    }
}
