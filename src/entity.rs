//! code associated with managing and composing entities

use crate::entity::EntitySlot::Empty;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// an error representing a failure to claim an entity ID
#[derive(Default, Debug)]
pub struct ClaimEntityIdError {
    detail: Option<&'static str>,
}

impl ClaimEntityIdError {
    /// create a new `ClaimEntityIdError` without details
    pub fn new() -> Self {
        Default::default()
    }

    /// create a new `ClaimEntityIdError` with details
    pub fn new_with_detail(detail: &'static str) -> Self {
        Self {
            detail: Some(detail),
        }
    }
}

impl Display for ClaimEntityIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.detail {
            Some(detail) => write!(f, "Failed to generate a new entity ID. {}", detail),
            None => write!(f, "Failed to generate a new entity ID."),
        }
    }
}

impl Error for ClaimEntityIdError {}

///
#[derive(Default, Debug)]
pub struct DiscardEntityIdError {
    entity_id: usize,
    detail: Option<&'static str>,
}

impl DiscardEntityIdError {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Display for DiscardEntityIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.detail {
            Some(detail) => write!(
                f,
                "Failed to discard entity ID {}. {}",
                self.entity_id, detail
            ),
            None => write!(f, "Failed to discard entity ID {}.", self.entity_id),
        }
    }
}

pub enum EntitySlot<T> {
    Claimed(T),
    Empty(usize),
}

pub trait EntityList {
    fn claim_entity_id(&mut self) -> Result<usize, ClaimEntityIdError>;
    fn discard_entity_id(&mut self, entity_id: usize) -> Result<(), DiscardEntityIdError>;
}

pub struct VecEntityList<T> {
    next_empty_slot: Option<usize>,
    last_empty_slot: Option<usize>,
    entity_list: Vec<EntitySlot<T>>,
}

impl<T> EntityList for VecEntityList<T> {
    fn claim_entity_id(&mut self) -> Result<usize, ClaimEntityIdError> {
        let claimed_slot = match self.next_empty_slot {
            Some(slot) => slot,
            None => {
                self.entity_list.push(Empty(0));
                let last_index = self.entity_list.len() - 1;
                if let Some(last_slot) = self.last_empty_slot {
                    if let Some(mutable_slot) = self.entity_list.get_mut(last_slot) {
                        *mutable_slot = Empty(last_index);
                    }
                }
                last_index
            }
        };
        self.next_empty_slot = match self.next_empty_slot {
            Some(next_empty_slot) => {
                match self.entity_list.get(next_empty_slot) {
                    Some(empty_slot) => match empty_slot {
                        Empty(next) => Some(*next),
                        EntitySlot::Claimed(_) => None, // this shouldn't happen
                    },
                    None => None,
                }
            }
            None => None,
        };
        Ok(claimed_slot)
    }

    fn discard_entity_id(&mut self, entity_id: usize) -> Result<(), DiscardEntityIdError> {
        todo!()
    }
}

pub struct ArrayEntityList<T, const SIZE: usize> {
    next_empty_slot: Option<usize>,
    entity_list: [EntitySlot<T>; SIZE],
}

impl<T, const SIZE: usize> EntityList for ArrayEntityList<T, SIZE> {
    fn claim_entity_id(&mut self) -> Result<usize, ClaimEntityIdError> {
        todo!()
    }

    fn discard_entity_id(&mut self, entity_id: usize) -> Result<(), DiscardEntityIdError> {
        todo!()
    }
}
