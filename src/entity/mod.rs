//! code associated with managing and composing entities

use crate::entity::component_flags::{ArrayComponentFlags, ComponentFlags, VecComponentFlags};
use crate::memory::parking_lot::array_parking_lot::ArrayParkingLot;
use crate::memory::parking_lot::error::{DeleteError, PutError};
use crate::memory::parking_lot::vec_parking_lot::VecParkingLot;
use crate::memory::parking_lot::ParkingLot;
use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{BitAnd, Shl};

pub mod component_flags;

#[derive(Clone, Copy, Debug)]
pub struct ComponentFlagId {
    pub set_index: usize,
    pub flag_index: usize,
}

pub struct ComponentRegistry<T>
where
    T: 'static,
{
    component_to_id_map: HashMap<TypeId, ComponentFlagId>,
    next_registration: ComponentFlagId,
    _phantom: PhantomData<T>,
}

impl<T> ComponentRegistry<T> {
    pub fn get_component_flag_id(&self) -> Option<ComponentFlagId> {
        let type_id = TypeId::of::<T>();
        self.component_to_id_map.get(&type_id).copied()
    }

    pub fn register_component<C>(&mut self) -> Option<ComponentFlagId> {
        let type_id = TypeId::of::<T>();
        let next_reg = self.get_next_registration();
        self.component_to_id_map.insert(type_id, next_reg)
    }

    pub fn get_next_registration(&mut self) -> ComponentFlagId {
        let reg = self.next_registration;
        match self.next_registration.flag_index {
            index if index < size_of::<T>() * 8 => {
                self.next_registration.flag_index += 1;
            }
            _ => {
                self.next_registration = ComponentFlagId {
                    set_index: self.next_registration.set_index + 1,
                    flag_index: 0,
                }
            }
        }
        reg
    }
}

#[derive(Clone, Debug)]
pub struct EntityStorage<T, P, F>
where
    T: Copy + BitAnd<T> + Shl<usize, Output = T> + From<u8>,
    <T as BitAnd<T>>::Output: PartialEq<T>,
    P: ParkingLot<T>,
    F: ComponentFlags<T>,
{
    entity_table: P,
    entity_table_extensions: Vec<F>,
    _phantom: PhantomData<T>,
}

impl<T, P, F> EntityStorage<T, P, F>
where
    T: Copy + BitAnd<T> + Shl<usize, Output = T> + From<u8>,
    <T as BitAnd<T>>::Output: PartialEq<T>,
    P: ParkingLot<T>,
    F: ComponentFlags<T>,
{
    pub fn create_entity(&mut self) -> Result<usize, PutError> {
        match self.entity_table.put(T::from(0)) {
            Ok(entity_id) => Ok(entity_id),
            Err(err) => Err(err),
        }
    }

    pub fn delete_entity(&mut self, entity_id: usize) -> Result<(), DeleteError> {
        match self.entity_table.delete(entity_id) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

pub type VecEntityStorage<T> = EntityStorage<T, VecParkingLot<T>, VecComponentFlags<T>>;

pub type ArrayEntityStorage<T, const SIZE: usize> =
    EntityStorage<T, ArrayParkingLot<T, SIZE>, ArrayComponentFlags<T, SIZE>>;
