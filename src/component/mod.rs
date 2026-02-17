//! code associated with component types and component storage

use error::ComponentWriteError;
use std::fmt::Debug;

pub mod archetype;
pub mod error;

/// a marker type representing a type that can be used as a component
pub trait Component: Sized + Debug {}
