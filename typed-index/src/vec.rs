mod into_iter;
pub use into_iter::IntoIter;

use crate::{
    macros::delegate,
    typed_index::{TypedIndex, TypedIndexCollection},
    TypedIndexBoxedSlice,
};

pub type TypedIndexVec<I, T> = TypedIndexCollection<I, Vec<T>>;

impl<I, T> TypedIndexVec<I, T>
where
    I: TypedIndex,
{
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn next_index(&self) -> I {
        I::from_usize(self.len())
    }

    #[inline]
    pub fn push(&mut self, value: T) -> I {
        let index = self.next_index();
        self.inner.push(value);
        index
    }

    #[inline]
    pub fn into_boxed_slice(self) -> TypedIndexBoxedSlice<I, T> {
        TypedIndexBoxedSlice::new(self.inner.into_boxed_slice())
    }

    delegate! { pub fn len(&self) -> usize }
    delegate! { pub fn capacity(&self) -> usize }
    delegate! { pub fn reserve(&mut self, additional: usize) }
    delegate! { pub fn reserve_exact(&mut self, additional: usize) }
    delegate! { pub fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> }
    delegate! { pub fn shrink_to_fit(&mut self) }
    delegate! { pub fn shrink_to(&mut self, min_capacity: usize) }
    delegate! { pub fn is_empty(&self) -> bool }
}

impl<I, T> IntoIterator for TypedIndexVec<I, T>
where
    I: TypedIndex,
{
    type Item = T;

    type IntoIter = IntoIter<I, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<I, T> std::ops::Index<I> for TypedIndexVec<I, T>
where
    I: TypedIndex,
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index.into_usize()]
    }
}

impl<I, T> std::ops::IndexMut<I> for TypedIndexVec<I, T>
where
    I: TypedIndex,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.inner[index.into_usize()]
    }
}
