use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub struct PutError {
    detail: Option<&'static str>,
}

impl PutError {
    pub fn new() -> Self {
        Self { detail: None }
    }

    pub fn new_with_detail(detail: &'static str) -> Self {
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

#[derive(Clone, Copy, Debug)]
pub struct DeleteError {
    index: usize,
    detail: Option<&'static str>,
}

impl DeleteError {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            detail: None,
        }
    }

    pub fn new_with_detail(index: usize, detail: &'static str) -> Self {
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
