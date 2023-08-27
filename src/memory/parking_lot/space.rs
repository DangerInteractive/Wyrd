//! code for storing individual memory cells in a parking lot data structure

use crate::memory::parking_lot::link::Link;
use crate::memory::parking_lot::space::Space::{Empty, Full};
use std::panic::panic_any;

/// a single memory cell in a parking lot, containing either a value, or a link
/// to another empty memory cell if it's empty (and another empty `Space`
/// existed when it was created)
#[derive(Clone, Copy, Debug)]
pub enum Space<T> {
    Full(T),
    Empty(Link),
}

impl<T> Space<T> {
    /// return the `Link` if space is empty, panic otherwise
    #[must_use]
    pub fn expect_empty(self, msg: &'static str) -> Link {
        match self {
            Empty(link) => link,
            Full(_) => panic_any(msg),
        }
    }

    /// return the value if space is full, panic otherwise
    #[must_use]
    pub fn expect_full(self, msg: &'static str) -> T {
        match self {
            Full(value) => value,
            Empty(_) => panic_any(msg),
        }
    }

    /// return an immutable reference to the `Link` if space is empty,
    /// panic otherwise
    #[must_use]
    pub fn expect_empty_ref(&self, msg: &'static str) -> &Link {
        match self {
            Empty(link) => link,
            Full(_) => panic_any(msg),
        }
    }

    /// return an immutable reference to the value if space is full,
    /// panic otherwise
    #[must_use]
    pub fn expect_full_ref(&self, msg: &'static str) -> &T {
        match self {
            Full(value) => value,
            Empty(_) => panic_any(msg),
        }
    }

    /// return a mutable reference to the `Link` if space is empty,
    /// panic otherwise
    #[must_use]
    pub fn expect_empty_mut(&mut self, msg: &'static str) -> &mut Link {
        match self {
            Empty(link) => link,
            Full(_) => panic_any(msg),
        }
    }

    /// return a mutable reference to the value if space is empty,
    /// panic otherwise
    #[must_use]
    pub fn expect_full_mut(&mut self, msg: &'static str) -> &mut T {
        match self {
            Full(value) => value,
            Empty(_) => panic_any(msg),
        }
    }

    /// return true if space is empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty(_))
    }

    /// return true if space is full, false otherwise
    #[must_use]
    pub fn is_full(&self) -> bool {
        matches!(self, Full(_))
    }
}
