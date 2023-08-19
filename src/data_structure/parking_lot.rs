use crate::data_structure::parking_lot::Space::{Empty, Full};
use std::array::from_fn;
use std::mem::{replace, swap};
use std::panic::panic_any;

#[derive(Clone, Copy, Debug)]
pub struct Link<I>(Option<I>);

impl<I> Link<I> {
    #[must_use]
    pub fn new(next: I) -> Self {
        Self(Some(next))
    }

    #[must_use]
    pub fn new_end() -> Self {
        Self(None)
    }

    #[must_use]
    pub fn push_new(current_index: I, next_link: &mut Self) -> Self {
        let mut new_link = Link::new(current_index);
        swap(&mut new_link, next_link);
        new_link
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Space<T, I> {
    Full(T),
    Empty(Link<I>),
}

impl<T, I> Space<T, I> {
    #[must_use]
    pub fn expect_empty(self, msg: &'static str) -> Link<I> {
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
    pub fn expect_empty_ref(&self, msg: &'static str) -> &Link<I> {
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
pub enum PutError {
    OutOfSpace,
}

#[derive(Clone, Copy, Debug)]
pub enum DeleteError {
    NotFound,
}

pub fn put_space<T, I>(space: &mut Space<T, I>, next_empty: &mut Link<I>, value: T) {
    *next_empty =
        replace(space, Full(value)).expect_empty("next_empty pointed to a non-empty space");
}

pub fn delete_space<T, I>(space: &mut Space<T, I>, next_empty: &mut Link<I>, index: I) {
    if space.is_full() {
        let link = Link::push_new(index, next_empty);
        *space = Empty(link);
    }
}

pub trait ParkingLot<T, I> {
    fn get(&self, index: I) -> Option<&T>;
    fn put(&mut self, value: T) -> Result<I, PutError>;
    fn delete(&mut self, index: I) -> Result<(), DeleteError>;
}

pub struct VecParkingLot<T> {
    next_empty: Link<usize>,
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
        match self.next_empty.0 {
            Some(index) => {
                put_space(&mut self.vec[index], &mut self.next_empty, value);
                Ok(index)
            }
            None => {
                self.vec.push(Full(value));
                Ok(self.vec.len() - 1)
            }
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), DeleteError> {
        match self.vec.get_mut(index) {
            Some(space) => {
                delete_space(space, &mut self.next_empty, index);
                Ok(())
            }
            None => Err(DeleteError::NotFound),
        }
    }
}

impl<T> Default for VecParkingLot<T> {
    fn default() -> Self {
        Self {
            next_empty: Link::new_end(),
            vec: vec![],
        }
    }
}

pub struct ArrayParkingLot<T, const SIZE: usize> {
    next_empty: Link<usize>,
    array: [Space<T, usize>; SIZE],
}

impl<T, const SIZE: usize> Default for ArrayParkingLot<T, SIZE> {
    fn default() -> Self {
        Self {
            next_empty: Link::new(0),
            array: from_fn(|index| match index {
                index if index == SIZE - 1 => Empty(Link::new_end()),
                index => Empty(Link::new(index + 1)),
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
        match self.next_empty.0 {
            Some(index) => {
                put_space(&mut self.array[index], &mut self.next_empty, value);
                Ok(index)
            }
            None => Err(PutError::OutOfSpace),
        }
    }

    fn delete(&mut self, index: usize) -> Result<(), DeleteError> {
        match self.array.get_mut(index) {
            Some(space) => {
                delete_space(space, &mut self.next_empty, index);
                Ok(())
            }
            None => Err(DeleteError::NotFound),
        }
    }
}
