use std::{iter::FusedIterator, marker::PhantomData};

use crate::{IndexableDoubleEndedIterator, IndexableIterator};

use super::*;

pub struct IntoIter<I, T> {
    // See https://users.rust-lang.org/t/determine-the-current-index-for-a-possibly-advanced-iterator/103501/7.
    inner: std::iter::Enumerate<std::vec::IntoIter<T>>,
    _index_marker: PhantomData<I>,
}

impl<I, T> IntoIter<I, T>
where
    I: TypedIndex,
{
    #[inline]
    pub fn new(vec: TypedIndexVec<I, T>) -> Self {
        Self {
            inner: vec.inner.into_iter().enumerate(),
            _index_marker: PhantomData,
        }
    }
}

fn drop_index<T>((_, t): (usize, T)) -> T {
    t
}

fn map_index<I, T>((i, t): (usize, T)) -> (I, T)
where
    I: TypedIndex,
{
    (I::from_usize(i), t)
}

impl<I, T> Iterator for IntoIter<I, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(drop_index)
    }

    delegate! { fn size_hint(&self) -> (usize, Option<usize>) }
    delegate! { fn count(self) -> usize }
}

impl<I, T> DoubleEndedIterator for IntoIter<I, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(drop_index)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n).map(drop_index)
    }
}

impl<I, T> ExactSizeIterator for IntoIter<I, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<I, T> FusedIterator for IntoIter<I, T> {}

impl<I, T> IndexableIterator for IntoIter<I, T>
where
    I: TypedIndex,
{
    type Index = I;

    #[inline]
    fn indexed_next(&mut self) -> Option<(Self::Index, Self::Item)> {
        self.inner.next().map(map_index)
    }
}

impl<I, T> IndexableDoubleEndedIterator for IntoIter<I, T>
where
    I: TypedIndex,
{
    #[inline]
    fn indexed_next_back(&mut self) -> Option<(Self::Index, Self::Item)> {
        self.inner.next_back().map(map_index)
    }
}
