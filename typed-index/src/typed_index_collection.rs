use std::marker::PhantomData;

use crate::{TypedIndex, TypedIndexSlice, TypedIndexSliceMut};

// TODO: Determine if we want this abstraction or just implement the distinct types directly.
pub struct TypedIndexCollection<X, T> {
    _marker: PhantomData<X>,
    pub(crate) inner: T,
}

impl<X, T> TypedIndexCollection<X, T> {
    #[inline]
    pub fn new(inner: T) -> Self {
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

impl<X, T> std::default::Default for TypedIndexCollection<X, T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            _marker: Default::default(),
            inner: Default::default(),
        }
    }
}
