use crate::data_structure::parking_lot::link::Link;
use crate::data_structure::parking_lot::space::Space::{Empty, Full};
use std::panic::panic_any;

#[derive(Clone, Copy, Debug)]
pub enum Space<T> {
    Full(T),
    Empty(Link),
}

impl<T> Space<T> {
    #[must_use]
    pub fn expect_empty(self, msg: &'static str) -> Link {
        match self {
            Empty(link) => link,
            Full(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn expect_full(self, msg: &'static str) -> T {
        match self {
            Full(value) => value,
            Empty(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn expect_empty_ref(&self, msg: &'static str) -> &Link {
        match self {
            Empty(link) => link,
            Full(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn expect_full_ref(&self, msg: &'static str) -> &T {
        match self {
            Full(value) => value,
            Empty(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn expect_empty_mut(&mut self, msg: &'static str) -> &mut Link {
        match self {
            Empty(link) => link,
            Full(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn expect_full_mut(&mut self, msg: &'static str) -> &mut T {
        match self {
            Full(value) => value,
            Empty(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty(_))
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        matches!(self, Full(_))
    }
}
