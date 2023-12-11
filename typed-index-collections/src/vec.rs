use crate::{macros::delegate, Indexable, TypedIndex, TypedIndexBoxedSlice, TypedIndexCollection};

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
        X::from(self.len())
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

    pub fn iter(&self) -> Indexable<X, std::slice::Iter<'_, T>> {
        Indexable::new_from_zero(self.inner.as_slice().iter())
    }

    pub fn iter_mut(&mut self) -> Indexable<X, std::slice::IterMut<'_, T>> {
        Indexable::new_from_zero(self.inner.as_mut_slice().iter_mut())
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

impl<'a, X, T> IntoIterator for &'a TypedIndexVec<X, T>
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

impl<'a, X, T> IntoIterator for &'a mut TypedIndexVec<X, T>
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

impl<X, T> FromIterator<T> for TypedIndexVec<X, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

// impl<X, T> std::ops::Index<X> for TypedIndexVec<X, T>
// where
//     X: TypedIndex,
// {
//     type Output = T;

//     #[inline]
//     fn index(&self, index: X) -> &Self::Output {
//         &self.inner[index.into()]
//     }
// }

// impl<X, T> std::ops::IndexMut<X> for TypedIndexVec<X, T>
// where
//     X: TypedIndex,
// {
//     #[inline]
//     fn index_mut(&mut self, index: X) -> &mut Self::Output {
//         &mut self.inner[index.into()]
//     }
// }
