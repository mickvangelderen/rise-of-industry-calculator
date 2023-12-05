use crate::{
    macros::delegate,
    typed_index::{TypedIndex, TypedIndexCollection},
    TypedIndexVec,
};

pub type TypedIndexBoxedSlice<I, T> = TypedIndexCollection<I, Box<[T]>>;

impl<I, T> TypedIndexBoxedSlice<I, T>
where
    I: TypedIndex,
{
    #[inline]
    pub fn into_vec(self) -> TypedIndexVec<I, T> {
        TypedIndexVec::new(self.inner.into_vec())
    }

    delegate! { pub fn len(&self) -> usize }
    delegate! { pub fn is_empty(&self) -> bool }
}

impl<I, T> IntoIterator for TypedIndexBoxedSlice<I, T>
where
    I: TypedIndex,
{
    type Item = T;

    type IntoIter = crate::vec::IntoIter<I, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        crate::vec::IntoIter::new(self.into_vec())
    }
}

impl<I, T> std::ops::Index<I> for TypedIndexBoxedSlice<I, T>
where
    I: TypedIndex,
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index.into_usize()]
    }
}

impl<I, T> std::ops::IndexMut<I> for TypedIndexBoxedSlice<I, T>
where
    I: TypedIndex,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.inner[index.into_usize()]
    }
}
