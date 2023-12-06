use std::{array::IntoIter, marker::PhantomData};

pub struct TypedIndexSlice<'a, X, T> {
    inner: &'a [T],
    index: X,
}

pub struct Iter<'a, X, T> {
    inner: std::slice::Iter<'a, T>,
    index: X,
}

impl<'a, X, T> Iter<'a, X, T> {
    pub(crate) fn new(inner: TypedIndexSlice<'a, X, T>) -> Self {
        Self {
            inner: inner.inner.iter(),
            index: inner.index,
        }
    }
}

impl<'a, X, T> Iterator for Iter<'a, X, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// TODO: All the iter impls.

impl<'a, X, T> IntoIterator for TypedIndexSlice<'a, X, T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, X, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, X, T> TypedIndexSlice<'a, X, T> {
    #[inline]
    pub fn iter(self) -> Iter<'a, X, T> {
        Iter::new(self)
    }
}
