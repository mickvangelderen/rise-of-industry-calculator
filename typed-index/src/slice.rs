use std::{array::IntoIter, marker::PhantomData};

pub struct TypedIndexSlice<'a, I, T> {
    inner: &'a [T],
    index: I,
}

pub struct Iter<'a, I, T> {
    inner: std::slice::Iter<'a, T>,
    index: I,
}

impl<'a, I, T> Iter<'a, I, T> {
    pub(crate) fn new(inner: TypedIndexSlice<'a, I, T>) -> Self {
        Self {
            inner: inner.inner.iter(),
            index: inner.index,
        }
    }
}

impl<'a, I, T> Iterator for Iter<'a, I, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// TODO: All the iter impls.

impl<'a, I, T> IntoIterator for TypedIndexSlice<'a, I, T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I, T> TypedIndexSlice<'a, I, T> {
    #[inline]
    pub fn iter(self) -> Iter<'a, I, T> {
        Iter::new(self)
    }
}
