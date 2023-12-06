use crate::{
    macros::delegate,
    typed_index::{TypedIndex, TypedIndexCollection},
    TypedIndexVec,
};

pub type TypedIndexBoxedSlice<X, T> = TypedIndexCollection<X, Box<[T]>>;

impl<X, T> TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    #[inline]
    pub fn into_vec(self) -> TypedIndexVec<X, T> {
        TypedIndexVec::new(self.inner.into_vec())
    }

    delegate! { pub fn len(&self) -> usize }
    delegate! { pub fn is_empty(&self) -> bool }
}

impl<X, T> IntoIterator for TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    type Item = T;

    type IntoIter = crate::vec::IntoIter<X, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        crate::vec::IntoIter::new(self.into_vec())
    }
}

impl<X, T> std::ops::Index<X> for TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    type Output = T;

    #[inline]
    fn index(&self, index: X) -> &Self::Output {
        &self.inner[index.into_usize()]
    }
}

impl<X, T> std::ops::IndexMut<X> for TypedIndexBoxedSlice<X, T>
where
    X: TypedIndex,
{
    #[inline]
    fn index_mut(&mut self, index: X) -> &mut Self::Output {
        &mut self.inner[index.into_usize()]
    }
}
