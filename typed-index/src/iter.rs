use std::iter::FusedIterator;

use crate::{TypedIndex, UntypedOffset};

mod private {
    /// Marker trait to restrict instantiations of Indexable/Indexed.
    pub trait IndexableIterator: Iterator {}

    impl<T> IndexableIterator for std::vec::IntoIter<T> {}
    impl<'a, T> IndexableIterator for std::slice::Iter<'a, T> {}
    impl<'a, T> IndexableIterator for std::slice::IterMut<'a, T> {}
}

use private::IndexableIterator;

pub struct Indexable<X, I> {
    inner: I,
    index: X,
}

impl<X, I> Indexable<X, I>
where
    X: TypedIndex,
    I: IndexableIterator,
{
    pub(crate) fn new(inner: I, index: X) -> Self {
        Self { inner, index }
    }

    pub(crate) fn new_from_zero(inner: I) -> Self {
        Self::new(inner, 0.into())
    }

    pub fn index(self) -> Indexed<X, I> {
        let Self { inner, index } = self;
        Indexed { inner, index }
    }
}

impl<X, I> Iterator for Indexable<X, I>
where
    X: TypedIndex,
    I: IndexableIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        self.index += UntypedOffset(1);
        Some(item)
    }
}

impl<X, I> DoubleEndedIterator for Indexable<X, I>
where
    X: TypedIndex,
    I: IndexableIterator + DoubleEndedIterator + ExactSizeIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.inner.next_back()?;
        Some(item)
    }
}

impl<X, I> ExactSizeIterator for Indexable<X, I>
where
    X: TypedIndex,
    I: IndexableIterator + ExactSizeIterator,
{
}

impl<X, I> FusedIterator for Indexable<X, I>
where
    X: TypedIndex,
    I: IndexableIterator + FusedIterator,
{
}

pub struct Indexed<X, I> {
    inner: I,
    index: X,
}

impl<X, I> Iterator for Indexed<X, I>
where
    X: TypedIndex,
    I: IndexableIterator,
{
    type Item = (X, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        let index = self.index;
        self.index += UntypedOffset(1);
        Some((index, item))
    }
}

impl<X, I> DoubleEndedIterator for Indexed<X, I>
where
    X: TypedIndex,
    I: IndexableIterator + DoubleEndedIterator + ExactSizeIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.inner.next_back()?;
        let index = self.index + UntypedOffset(self.inner.len());
        Some((index, item))
    }
}

impl<X, I> ExactSizeIterator for Indexed<X, I>
where
    X: TypedIndex,
    I: IndexableIterator + ExactSizeIterator,
{
}

impl<X, I> FusedIterator for Indexed<X, I>
where
    X: TypedIndex,
    I: IndexableIterator + FusedIterator,
{
}
