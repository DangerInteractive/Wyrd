use std::mem::swap;

#[derive(Clone, Copy, Debug)]
pub struct Link(pub(crate) Option<usize>);

impl Link {
    #[must_use]
    pub fn new(next: usize) -> Self {
        Self(Some(next))
    }

    #[must_use]
    pub fn new_end() -> Self {
        Self(None)
    }

    #[must_use]
    pub fn push_new(current_index: usize, next_link: &mut Self) -> Self {
        let mut new_link = Link::new(current_index);
        swap(&mut new_link, next_link);
        new_link
    }
}
