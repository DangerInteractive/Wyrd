use std::any::type_name;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// an error representing a failure to store or write to a component
#[derive(Debug)]
pub struct ComponentWriteError {
    component_type: &'static str,
    entity_id: usize,
    detail: Option<&'static str>,
}

impl ComponentWriteError {
    /// create a new `ComponentWriteError` given the ID of the associated entity
    /// (without further details)
    pub fn new<T>(entity_id: usize) -> Self {
        Self {
            component_type: type_name::<T>(),
            entity_id,
            detail: None,
        }
    }

    /// create a new `ComponentWriteError` given the ID of the associated entity,
    /// and details
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
