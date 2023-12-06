use std::{iter::FusedIterator, marker::PhantomData};

use crate::{IndexableDoubleEndedIterator, IndexableIterator};

use super::*;

pub struct IntoIter<X, T> {
    // See https://users.rust-lang.org/t/determine-the-current-index-for-a-possibly-advanced-iterator/103501/7.
    inner: std::iter::Enumerate<std::vec::IntoIter<T>>,
    _index_marker: PhantomData<X>,
}

impl<X, T> IntoIter<X, T>
where
    X: TypedIndex,
{
    #[inline]
    pub fn new(vec: TypedIndexVec<X, T>) -> Self {
        Self {
            inner: vec.inner.into_iter().enumerate(),
            _index_marker: PhantomData,
        }
    }
}

fn drop_index<T>((_, t): (usize, T)) -> T {
    t
}

fn map_index<X, T>((X, t): (usize, T)) -> (X, T)
where
    X: TypedIndex,
{
    (X::from_usize(X), t)
}

impl<X, T> Iterator for IntoIter<X, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(drop_index)
    }

    delegate! { fn size_hint(&self) -> (usize, Option<usize>) }
    delegate! { fn count(self) -> usize }
}

impl<X, T> DoubleEndedIterator for IntoIter<X, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(drop_index)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n).map(drop_index)
    }
}

impl<X, T> ExactSizeIterator for IntoIter<X, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<X, T> FusedIterator for IntoIter<X, T> {}

impl<X, T> IndexableIterator for IntoIter<X, T>
where
    X: TypedIndex,
{
    type Index = X;

    #[inline]
    fn indexed_next(&mut self) -> Option<(Self::Index, Self::Item)> {
        self.inner.next().map(map_index)
    }
}

impl<X, T> IndexableDoubleEndedIterator for IntoIter<X, T>
where
    X: TypedIndex,
{
    #[inline]
    fn indexed_next_back(&mut self) -> Option<(Self::Index, Self::Item)> {
        self.inner.next_back().map(map_index)
    }
}
