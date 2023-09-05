//! code for linking empty memory cells in a depot data structure

use std::mem::swap;

/// a value that links empty spaces in a depot together for quick location
/// of empty spaces after insertion/deletion
#[derive(Clone, Copy, Debug)]
pub struct Link(pub Option<usize>);

impl Link {
    /// create a new `Link` given the index of the next empty space
    #[must_use]
    pub const fn new(next: usize) -> Self {
        Self(Some(next))
    }

    /// create a new `Link` that ends the chain
    /// (without the index of a next empty space, it's the last one)
    #[must_use]
    pub const fn new_end() -> Self {
        Self(None)
    }

    /// create a new `Link` for a given index and link to it in an existing
    /// chain of links
    #[must_use]
    pub fn push_new(current_index: usize, next_link: &mut Self) -> Self {
        let mut new_link = Link::new(current_index);
        swap(&mut new_link, next_link);
        new_link
    }
}
