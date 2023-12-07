use std::marker::PhantomData;

use crate::TypedIndexCollection;
use crate::{macros::delegate, typed_index::TypedIndex, Indexable, TypedIndexVec};

pub type TypedIndexBoxedSlice<X, T> = TypedIndexCollection<X, Box<[T]>>;

impl<X, T> TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    pub fn into_vec(self) -> TypedIndexVec<X, T> {
        TypedIndexVec::new(self.inner.into_vec())
    }

    pub fn iter(&self) -> Indexable<X, std::slice::Iter<'_, T>> {
        Indexable::new_from_zero(self.inner.iter())
    }

    pub fn iter_mut(&mut self) -> Indexable<X, std::slice::IterMut<'_, T>> {
        Indexable::new_from_zero(self.inner.iter_mut())
    }

    delegate! { pub fn len(&self) -> usize }
    delegate! { pub fn is_empty(&self) -> bool }
}

impl<X, T> IntoIterator for TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    type Item = T;

    type IntoIter = Indexable<X, std::vec::IntoIter<Self::Item>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<'a, X, T> IntoIterator for &'a TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    type Item = &'a T;

    type IntoIter = Indexable<X, std::slice::Iter<'a, T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, X, T> IntoIterator for &'a mut TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    type Item = &'a mut T;

    type IntoIter = Indexable<X, std::slice::IterMut<'a, T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<X, T> std::ops::Index<X> for TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    type Output = T;

    #[inline]
    fn index(&self, index: X) -> &Self::Output {
        &self.inner[index.into()]
    }
}

impl<X, T> std::ops::IndexMut<X> for TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    #[inline]
    fn index_mut(&mut self, index: X) -> &mut Self::Output {
        &mut self.inner[index.into()]
    }
}
