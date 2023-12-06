use crate::{Indexable, TypedIndex};

pub struct TypedIndexSlice<'a, X, T> {
    inner: &'a [T],
    index: X,
}

impl<'a, X, T> IntoIterator for TypedIndexSlice<'a, X, T>
where
    X: TypedIndex,
{
    type Item = &'a T;

    type IntoIter = Indexable<X, std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, X, T> TypedIndexSlice<'a, X, T>
where
    X: TypedIndex,
{
    #[inline]
    pub fn iter(self) -> Indexable<X, std::slice::Iter<'a, T>> {
        Indexable::new(self.inner.iter(), self.index)
    }
}

pub struct TypedIndexSliceMut<'a, X, T> {
    inner: &'a mut [T],
    index: X,
}

impl<'a, X, T> IntoIterator for TypedIndexSliceMut<'a, X, T>
where
    X: TypedIndex,
{
    type Item = &'a mut T;

    type IntoIter = Indexable<X, std::slice::IterMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, X, T> TypedIndexSliceMut<'a, X, T>
where
    X: TypedIndex,
{
    #[inline]
    pub fn iter_mut(self) -> Indexable<X, std::slice::IterMut<'a, T>> {
        Indexable::new(self.inner.iter_mut(), self.index)
    }
}
