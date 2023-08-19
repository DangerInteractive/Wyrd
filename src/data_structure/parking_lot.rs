use crate::data_structure::parking_lot::Space::{Empty, Full};
use std::array::from_fn;
use std::panic::panic_any;

#[derive(Clone, Copy, Debug)]
pub struct Link<I> {
    pub prev: Option<I>,
    pub next: Option<I>,
}

impl<I> Link<I> {
    #[must_use]
    pub fn new(prev: Option<I>, next: Option<I>) -> Self {
        Self { prev, next }
    }

    #[must_use]
    pub fn new_middle(prev: I, next: I) -> Self {
        Self {
            prev: Some(prev),
            next: Some(next),
        }
    }

    #[must_use]
    pub fn new_start(next: I) -> Self {
        Self {
            prev: None,
            next: Some(next),
        }
    }

    #[must_use]
    pub fn new_end(next: I) -> Self {
        Self {
            prev: Some(next),
            next: None,
        }
    }

    #[must_use]
    pub fn new_detached() -> Self {
        Self {
            prev: None,
            next: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Space<T, I> {
    Full(T),
    Empty(Link<I>),
}

impl<T, I> Space<T, I> {
    #[must_use]
    pub fn expect_empty(&self, msg: &'static str) -> &Link<I> {
        match self {
            Empty(link) => link,
            Full(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn expect_full(&self, msg: &'static str) -> &T {
        match self {
            Full(value) => value,
            Empty(_) => panic_any(msg),
        }
    }

    #[must_use]
    pub fn expect_empty_mut(&mut self, msg: &'static str) -> &mut Link<I> {
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

#[derive(Clone, Copy, Debug)]
pub struct EmptyBounds<I> {
    pub first: I,
    pub last: I,
}

impl<I> EmptyBounds<I> {
    #[must_use]
    pub fn new(first: I, last: I) -> Self {
        Self { first, last }
    }
}

impl<I: Copy> EmptyBounds<I> {
    #[must_use]
    pub fn new_single(index: I) -> Self {
        Self {
            first: index,
            last: index,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PutError {
    OutOfSpace,
}

#[derive(Clone, Copy, Debug)]
pub enum DeleteError {
    NotFound,
}

#[must_use]
pub fn find_last_empty_index<T>(slice: &[Space<T, usize>], offset: usize) -> Option<usize> {
    slice
        .iter()
        .rev()
        .position(|space| space.is_empty())
        .map(move |i| i + offset)
}

#[must_use]
pub fn find_first_empty_index<T>(slice: &[Space<T, usize>], offset: usize) -> Option<usize> {
    slice
        .iter()
        .position(|space| space.is_empty())
        .map(move |i| i + offset)
}

#[must_use]
pub fn create_link_from_last_empty<T>(
    slice: &[Space<T, usize>],
    offset: usize,
) -> Option<Link<usize>> {
    match find_last_empty_index(slice, offset) {
        Some(last_empty_index) => slice.get(last_empty_index).map(|last_empty_space| {
            Link::new(
                Some(last_empty_index),
                last_empty_space.expect_empty("link was poisoned").next,
            )
        }),
        None => None,
    }
}

#[must_use]
pub fn create_link_from_first_empty<T>(
    slice: &[Space<T, usize>],
    offset: usize,
) -> Option<Link<usize>> {
    match find_first_empty_index(slice, offset) {
        Some(first_empty_index) => slice.get(first_empty_index).map(|first_empty_space| {
            Link::new(
                first_empty_space.expect_empty("link was poisoned").prev,
                Some(first_empty_index),
            )
        }),
        None => None,
    }
}

#[must_use]
pub fn create_link<T>(
    slice: &[Space<T, usize>],
    empty_bounds: &Option<EmptyBounds<usize>>,
    index: usize,
) -> Link<usize> {
    if empty_bounds.is_none() {
        return Link::new_detached();
    }

    match empty_bounds {
        Some(empty_bounds) => match index {
            index if index <= empty_bounds.first => Link::new_start(empty_bounds.first),
            index if index >= empty_bounds.last => Link::new_end(empty_bounds.last),
            index => {
                let empty_span = empty_bounds.last - empty_bounds.first;
                let next_start = index + 1;
                let prev_slice = &slice[empty_bounds.first..index];
                let next_slice = &slice[next_start..empty_bounds.last];
                match index {
                    index if index < (empty_span / 2 + empty_bounds.first) => {
                        create_link_from_last_empty(prev_slice, 0)
                            .or_else(move || create_link_from_first_empty(next_slice, next_start))
                            .unwrap_or_else(Link::new_detached)
                    }
                    _ => create_link_from_first_empty(next_slice, next_start)
                        .or_else(move || create_link_from_last_empty(prev_slice, 0))
                        .unwrap_or_else(Link::new_detached),
                }
            }
        },
        None => Link::new_detached(),
    }
}

pub fn join_neighboring_links<T>(
    slice: &mut [Space<T, usize>],
    empty_bounds: &mut Option<EmptyBounds<usize>>,
    center_link: Link<usize>,
) {
    // if center_link was the last empty space, mark the whole thing as full and be done
    if let (None, None) = (center_link.prev, center_link.next) {
        *empty_bounds = None;
        return;
    }

    if let Some(prev_index) = center_link.prev {
        let prev_space = slice
            .get_mut(prev_index)
            .expect("cannot follow index in link");
        prev_space
            .expect_empty_mut("link pointed to non-empty space")
            .next = center_link.next;
    }

    if let Some(next_index) = center_link.next {
        let next_space = slice
            .get_mut(next_index)
            .expect("cannot follow index in link");
        next_space
            .expect_empty_mut("link pointed to non-empty space")
            .prev = center_link.prev;
    }
}

pub fn split_neighboring_links<T>(
    slice: &mut [Space<T, usize>],
    empty_bounds: &mut Option<EmptyBounds<usize>>,
    center_index: usize,
    center_link: &Link<usize>,
) {
    // if empty_bounds is not set, then this is the only empty space
    if empty_bounds.is_none() {
        *empty_bounds = Some(EmptyBounds::new_single(center_index));
        return;
    }

    if let Some(prev_index) = center_link.prev {
        let prev_space = slice
            .get_mut(prev_index)
            .expect("cannot follow index in link");
        prev_space
            .expect_empty_mut("link pointed to non-empty space")
            .next = Some(center_index);
    }

    if let Some(next_index) = center_link.next {
        let next_space = slice
            .get_mut(next_index)
            .expect("cannot follow index in link");
        next_space
            .expect_empty_mut("link pointed to non-empty space")
            .prev = Some(center_index);
    }
}

pub fn delete_value_in_slice<T>(
    slice: &mut [Space<T, usize>],
    empty_bounds: &mut Option<EmptyBounds<usize>>,
    index: usize,
) {
    if let Full(_) = slice.get(index).expect("cannot read at the given index") {
        let link = create_link(slice, empty_bounds, index);
        split_neighboring_links(slice, empty_bounds, index, &link);

        let space = slice
            .get_mut(index)
            .expect("cannot write at the given index");
        *space = Empty(link);
    }
}

pub fn put_value_in_slice<T>(
    slice: &mut [Space<T, usize>],
    empty_bounds: &mut Option<EmptyBounds<usize>>,
    index: usize,
    value: T,
) {
    let space = slice
        .get_mut(index)
        .expect("cannot write to the given index");

    match space {
        Empty(link) => {
            let old_link = *link;
            *space = Full(value);
            join_neighboring_links(slice, empty_bounds, old_link);
        }
        Full(in_slice) => {
            *in_slice = value;
        }
    }
}

pub trait ParkingLot<T, I> {
    fn get(&self, index: I) -> Option<&T>;
    fn put(&mut self, value: T) -> Result<I, PutError>;
    fn delete(&mut self, index: I) -> Result<(), DeleteError>;
}

pub struct VecParkingLot<T> {
    empty_bounds: Option<EmptyBounds<usize>>,
    vec: Vec<Space<T, usize>>,
}

impl<T> ParkingLot<T, usize> for VecParkingLot<T> {
    fn get(&self, index: usize) -> Option<&T> {
        if let Some(Full(value)) = self.vec.get(index) {
            return Some(value);
        }
        None
    }

    fn put(&mut self, value: T) -> Result<usize, PutError> {
        match self.empty_bounds {
            Some(empty_bounds) => {
                let index = empty_bounds.first;
                put_value_in_slice(
                    self.vec.as_mut_slice(),
                    &mut self.empty_bounds,
                    index,
                    value,
                );
                Ok(index)
            }
            None => {
                self.vec.push(Full(value));
                Ok(self.vec.len() - 1)
            }
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), DeleteError> {
        match self.vec.get(index) {
            Some(space) => {
                if space.is_full() {
                    delete_value_in_slice(self.vec.as_mut_slice(), &mut self.empty_bounds, index);
                }
                Ok(())
            }
            None => Err(DeleteError::NotFound),
        }
    }
}

impl<T> Default for VecParkingLot<T> {
    fn default() -> Self {
        Self {
            empty_bounds: None,
            vec: vec![],
        }
    }
}

pub struct ArrayParkingLot<T, const SIZE: usize> {
    empty_bounds: Option<EmptyBounds<usize>>,
    array: [Space<T, usize>; SIZE],
}

impl<T, const SIZE: usize> Default for ArrayParkingLot<T, SIZE> {
    fn default() -> Self {
        Self {
            empty_bounds: Some(EmptyBounds {
                first: 0,
                last: SIZE - 1,
            }),
            array: from_fn(|index| match index {
                index if index == 0 => Empty(Link::new_start(1)),
                index if index == SIZE - 1 => Empty(Link::new_end(SIZE - 2)),
                index => Empty(Link::new_middle(index - 1, index + 1)),
            }),
        }
    }
}

impl<T, const SIZE: usize> ParkingLot<T, usize> for ArrayParkingLot<T, SIZE> {
    fn get(&self, index: usize) -> Option<&T> {
        if let Some(Full(value)) = self.array.get(index) {
            return Some(value);
        }
        None
    }

    fn put(&mut self, value: T) -> Result<usize, PutError> {
        match self.empty_bounds {
            Some(empty_bounds) => {
                let index = empty_bounds.first;
                put_value_in_slice(
                    self.array.as_mut_slice(),
                    &mut self.empty_bounds,
                    index,
                    value,
                );
                Ok(index)
            }
            None => Err(PutError::OutOfSpace),
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), DeleteError> {
        match self.array.get(index) {
            Some(space) => {
                if space.is_full() {
                    delete_value_in_slice(self.array.as_mut_slice(), &mut self.empty_bounds, index);
                }
                Ok(())
            }
            None => Err(DeleteError::NotFound),
        }
    }
}
