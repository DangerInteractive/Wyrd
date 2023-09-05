//! errors associated with the depot data structure

use std::error::Error;
use std::fmt::{Display, Formatter};

/// an error representing a failure to put a value into a depot
#[derive(Clone, Copy, Debug, Default)]
pub struct PutError {
    detail: Option<&'static str>,
}

impl PutError {
    /// create a new `PutError` without any details
    pub const fn new() -> Self {
        Self { detail: None }
    }

    /// create a new `PutError` with details
    pub const fn new_with_detail(detail: &'static str) -> Self {
        Self {
            detail: Some(detail),
        }
    }
}

impl Display for PutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.detail {
            Some(detail) => write!(f, "Unable to write a value into a space: {}", detail),
            None => write!(f, "Unable to write a value into a space."),
        }
    }
}

impl Error for PutError {}

/// an error representing a failure to delete a value from a depot
#[derive(Clone, Copy, Debug)]
pub struct DeleteError {
    index: usize,
    detail: Option<&'static str>,
}

impl DeleteError {
    /// create a new `DeleteError` given only the attempted index
    /// (without further details)
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            detail: None,
        }
    }

    /// create a new `DeleteError` given the attempted index and details
    pub const fn new_with_detail(index: usize, detail: &'static str) -> Self {
        Self {
            index,
            detail: Some(detail),
        }
    }
}

impl Display for DeleteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.detail {
            Some(detail) => write!(
                f,
                "Failed to delete from space at index {}: {}",
                self.index, detail
            ),
            None => write!(f, "Failed to delete from space at index {}.", self.index),
        }
    }
}

impl Error for DeleteError {}
