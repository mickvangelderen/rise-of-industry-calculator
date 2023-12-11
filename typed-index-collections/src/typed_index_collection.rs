use std::marker::PhantomData;

use crate::{TypedIndex, TypedIndexSlice, TypedIndexSliceMut};

// TODO: Determine if we want this abstraction or just implement the distinct types directly.
pub struct TypedIndexCollection<X, C> {
    _marker: PhantomData<X>,
    pub(crate) inner: C,
}

impl<X, C> TypedIndexCollection<X, C> {
    #[inline]
    pub fn new(inner: C) -> Self {
        Self {
            _marker: PhantomData,
            inner,
        }
    }
}

impl<X, C, T> TypedIndexCollection<X, C>
where
    X: TypedIndex,
    C: std::ops::Deref<Target = [T]>,
{
    pub fn as_slice(&self) -> TypedIndexSlice<'_, X, T> {
        TypedIndexSlice::new_from_zero(&self.inner)
    }
}

impl<X, C, T> TypedIndexCollection<X, C>
where
    X: TypedIndex,
    C: std::ops::DerefMut<Target = [T]>,
{
    pub fn as_mut_slice(&mut self) -> TypedIndexSliceMut<'_, X, T> {
        TypedIndexSliceMut::new_from_zero(&mut self.inner)
    }
}

impl<X, C> std::default::Default for TypedIndexCollection<X, C>
where
    C: Default,
{
    fn default() -> Self {
        Self {
            _marker: Default::default(),
            inner: Default::default(),
        }
    }
}

impl<X, C, T> std::ops::Index<X> for TypedIndexCollection<X, C>
where
    C: std::ops::Deref<Target = [T]>,
    X: TypedIndex,
{
    type Output = T;

    fn index(&self, index: X) -> &Self::Output {
        &self.inner[index.into()]
    }
}

impl<X, C, T> std::ops::IndexMut<X> for TypedIndexCollection<X, C>
where
    C: std::ops::DerefMut<Target = [T]>,
    X: TypedIndex,
{
    fn index_mut(&mut self, index: X) -> &mut Self::Output {
        &mut self.inner[index.into()]
    }
}
