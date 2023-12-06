use crate::{
    macros::delegate,
    typed_index::{TypedIndex, TypedIndexCollection},
    Indexable, TypedIndexBoxedSlice,
};

pub type TypedIndexVec<X, T> = TypedIndexCollection<X, Vec<T>>;

impl<X, T> TypedIndexVec<X, T>
where
    X: TypedIndex,
{
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn next_index(&self) -> X {
        X::from_usize(self.len())
    }

    #[inline]
    pub fn push(&mut self, value: T) -> X {
        let index = self.next_index();
        self.inner.push(value);
        index
    }

    #[inline]
    pub fn into_boxed_slice(self) -> TypedIndexBoxedSlice<X, T> {
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

impl<X, T> IntoIterator for TypedIndexVec<X, T>
where
    X: TypedIndex,
{
    type Item = T;

    type IntoIter = Indexable<X, std::vec::IntoIter<T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Indexable::new_from_zero(self.inner.into_iter())
    }
}

impl<X, T> std::ops::Index<X> for TypedIndexVec<X, T>
where
    X: TypedIndex,
{
    type Output = T;

    #[inline]
    fn index(&self, index: X) -> &Self::Output {
        &self.inner[index.into_usize()]
    }
}

impl<X, T> std::ops::IndexMut<X> for TypedIndexVec<X, T>
where
    X: TypedIndex,
{
    #[inline]
    fn index_mut(&mut self, index: X) -> &mut Self::Output {
        &mut self.inner[index.into_usize()]
    }
}
